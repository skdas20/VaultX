//! Vault file storage operations.
//!
//! Handles reading and writing the vault file with atomic operations.

use crate::error::CliError;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use vx_core::crypto::{derive_key, KEY_SIZE, SALT_SIZE};
use vx_core::{vault, Vault};

/// Default vault directory name
const VAULT_DIR: &str = ".vaultx";

/// Default vault file name
const VAULT_FILE: &str = "vault.vx";

/// Header size (magic + version + reserved)
const HEADER_SIZE: usize = 16;

/// Returns the path to the vault directory.
pub fn vault_dir() -> Result<PathBuf, CliError> {
    let home = dirs::home_dir().ok_or_else(|| {
        CliError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not determine home directory",
        ))
    })?;

    Ok(home.join(VAULT_DIR))
}

/// Returns the path to the vault file.
pub fn vault_path() -> Result<PathBuf, CliError> {
    Ok(vault_dir()?.join(VAULT_FILE))
}

/// Checks if the vault file exists.
pub fn vault_exists() -> Result<bool, CliError> {
    Ok(vault_path()?.exists())
}

/// Extracts the salt from a vault file without decrypting.
pub fn extract_salt() -> Result<[u8; SALT_SIZE], CliError> {
    let path = vault_path()?;
    let data = fs::read(&path)?;

    if data.len() < HEADER_SIZE + SALT_SIZE {
        return Err(CliError::Vault(vx_core::VaultError::CorruptedVault));
    }

    let salt: [u8; SALT_SIZE] = data[HEADER_SIZE..HEADER_SIZE + SALT_SIZE]
        .try_into()
        .map_err(|_| CliError::Vault(vx_core::VaultError::CorruptedVault))?;

    Ok(salt)
}

/// Derives the encryption key from password using the vault's salt.
pub fn derive_vault_key(password: &[u8]) -> Result<[u8; KEY_SIZE], CliError> {
    let salt = extract_salt()?;
    derive_key(password, &salt).map_err(CliError::Crypto)
}

/// Loads the vault from disk.
pub fn load_vault(password: &[u8]) -> Result<Vault, CliError> {
    let path = vault_path()?;

    if !path.exists() {
        return Err(CliError::VaultNotFound);
    }

    let data = fs::read(&path)?;
    vault::load_vault(&data, password).map_err(CliError::Vault)
}

/// Loads the vault and returns both the vault and the derived encryption key.
pub fn load_vault_with_key(password: &[u8]) -> Result<(Vault, [u8; KEY_SIZE]), CliError> {
    let path = vault_path()?;

    if !path.exists() {
        return Err(CliError::VaultNotFound);
    }

    let data = fs::read(&path)?;

    // Extract salt from file
    if data.len() < HEADER_SIZE + SALT_SIZE {
        return Err(CliError::Vault(vx_core::VaultError::CorruptedVault));
    }

    let salt: [u8; SALT_SIZE] = data[HEADER_SIZE..HEADER_SIZE + SALT_SIZE]
        .try_into()
        .map_err(|_| CliError::Vault(vx_core::VaultError::CorruptedVault))?;

    // Derive key
    let key = derive_key(password, &salt).map_err(CliError::Crypto)?;

    // Load vault
    let vault = vault::load_vault(&data, password).map_err(CliError::Vault)?;

    Ok((vault, key))
}

/// Loads vault using cached password if available, otherwise prompts.
pub fn load_vault_auto() -> Result<Vault, CliError> {
    use crate::session;

    // Try cached password first
    if let Some(cached_password) = session::get_cached_password()? {
        match load_vault(&cached_password) {
            Ok(vault) => return Ok(vault),
            Err(_) => {
                // Cache is stale, clear it
                let _ = session::clear_cached_password();
            }
        }
    }

    // Fall back to prompting
    let password = crate::input::read_password("Enter master password: ")?;
    load_vault(password.as_bytes())
}

/// Loads vault with key using cached password if available.
pub fn load_vault_with_key_auto() -> Result<(Vault, [u8; KEY_SIZE]), CliError> {
    use crate::session;

    // Try cached password first
    if let Some(cached_password) = session::get_cached_password()? {
        match load_vault_with_key(&cached_password) {
            Ok(result) => return Ok(result),
            Err(_) => {
                // Cache is stale, clear it
                let _ = session::clear_cached_password();
            }
        }
    }

    // Fall back to prompting
    let password = crate::input::read_password("Enter master password: ")?;
    load_vault_with_key(password.as_bytes())
}

/// Saves the vault to disk using atomic write.
///
/// # Security
/// Uses write-to-temp-then-rename pattern to prevent corruption
/// from interrupted writes.
/// 
/// For existing vaults, preserves the original salt to ensure
/// consistent encryption key derivation.
pub fn save_vault(vault: &Vault, password: &[u8]) -> Result<(), CliError> {
    let path = vault_path()?;
    let dir = vault_dir()?;

    // Ensure directory exists. One created by an older version may still be
    // readable by other users, so tighten it too.
    if !dir.exists() {
        create_private_dir(&dir)?;
    } else {
        restrict_permissions(&dir, 0o700)?;
    }

    // Extract existing salt if vault exists, otherwise None for new vault
    let existing_salt = if path.exists() {
        restrict_permissions(&path, 0o600)?;
        Some(extract_salt()?)
    } else {
        None
    };

    // Serialize and encrypt, preserving salt if it exists
    let data = if let Some(salt) = existing_salt {
        vault::save_vault_with_salt(vault, password, Some(&salt))?
    } else {
        vault::save_vault(vault, password)?
    };

    // Atomic write: write to temp file, then rename
    let temp_path = path.with_extension("tmp");

    // A leftover temp file from an interrupted write would make create_new fail
    if temp_path.exists() {
        fs::remove_file(&temp_path)?;
    }
    write_private_file(&temp_path, &data)?;

    // Rename temp to final (atomic on most filesystems)
    fs::rename(&temp_path, &path)?;

    Ok(())
}

/// Creates `dir` and any missing parents, readable only by the owner on Unix.
fn create_private_dir(dir: &Path) -> std::io::Result<()> {
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }

    builder.create(dir)
}

/// Sets an existing `path` to `mode` on Unix when its current mode allows
/// group or other access. Does nothing on other platforms.
fn restrict_permissions(path: &Path, mode: u32) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if fs::metadata(path)?.permissions().mode() & 0o077 != 0 {
            fs::set_permissions(path, fs::Permissions::from_mode(mode))?;
        }
    }

    #[cfg(not(unix))]
    let _ = (path, mode);

    Ok(())
}

/// Writes `data` to a new file at `path`, readable only by the owner on Unix.
///
/// The mode is set when the file is created, so it is never readable by
/// other users, not even briefly. Fails if `path` already exists.
fn write_private_file(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }

    let mut file = options.open(path)?;
    file.write_all(data)?;
    file.sync_all()
}

/// Creates a new vault file and returns the vault with its encryption key.
pub fn create_vault(password: &[u8]) -> Result<(Vault, [u8; KEY_SIZE]), CliError> {
    let vault = Vault::new();
    save_vault(&vault, password)?;

    // Now load to get the key (salt was just generated)
    let key = derive_vault_key(password)?;

    Ok((vault, key))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn mode(path: &Path) -> u32 {
        fs::metadata(path).unwrap().permissions().mode() & 0o777
    }

    #[test]
    fn private_dir_is_owner_only() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("home").join(VAULT_DIR);

        create_private_dir(&dir).unwrap();

        assert_eq!(mode(&dir), 0o700);
    }

    #[test]
    fn existing_open_dir_and_vault_are_restricted() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join(VAULT_DIR);
        let path = dir.join(VAULT_FILE);
        fs::create_dir(&dir).unwrap();
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o755)).unwrap();
        fs::write(&path, b"data").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();

        restrict_permissions(&dir, 0o700).unwrap();
        restrict_permissions(&path, 0o600).unwrap();

        assert_eq!(mode(&dir), 0o700);
        assert_eq!(mode(&path), 0o600);
    }

    #[test]
    fn owner_only_mode_is_left_alone() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join(VAULT_FILE);
        fs::write(&path, b"data").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o400)).unwrap();

        restrict_permissions(&path, 0o600).unwrap();

        assert_eq!(mode(&path), 0o400);
    }

    #[test]
    fn private_file_is_owner_only() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("vault.vx.tmp");

        write_private_file(&path, b"data").unwrap();

        assert_eq!(mode(&path), 0o600);
        assert_eq!(fs::read(&path).unwrap(), b"data");
    }

    #[test]
    fn private_file_refuses_existing_path() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("vault.vx.tmp");
        fs::write(&path, b"old").unwrap();

        assert!(write_private_file(&path, b"new").is_err());
    }
}
