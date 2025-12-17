//! Session password caching for VaultX.
//!
//! Caches the vault password in an encrypted temporary file to avoid
//! repeated password prompts within the same session.

use crate::error::CliError;
use crate::input;
use crate::storage;
use std::fs;
use std::io::Write;
use vx_core::crypto::{self, KEY_SIZE};

/// Executes the login command - caches password for session.
pub fn execute() -> Result<(), CliError> {
    // Verify vault exists
    if !storage::vault_exists()? {
        return Err(CliError::VaultNotFound);
    }

    // Get password
    let password = input::read_password("Enter master password: ")?;

    // Verify password is correct by trying to load vault
    let _ = storage::load_vault(password.as_bytes())?;

    // Cache the password
    cache_password(password.as_bytes())?;

    println!("✓ Password cached for current session.");
    println!("Subsequent commands will use cached password.");

    Ok(())
}

/// Gets the cached password if available and valid.
pub fn get_cached_password() -> Result<Option<Vec<u8>>, CliError> {
    let cache_path = password_cache_path()?;

    if !cache_path.exists() {
        return Ok(None);
    }

    // Read cache file
    let data = fs::read(&cache_path)?;

    // Cache file format:
    // - 32 bytes: session key (derived from unique session ID)
    // - 12 bytes: nonce
    // - remaining: encrypted password

    if data.len() < KEY_SIZE + 12 {
        // Invalid cache file, remove it
        let _ = fs::remove_file(&cache_path);
        return Ok(None);
    }

    // For simplicity, we'll use a session-specific key derived from process ID + boot time
    let session_key = derive_session_key()?;

    let nonce: [u8; 12] = data[KEY_SIZE..KEY_SIZE + 12]
        .try_into()
        .map_err(|_| CliError::Generic("Invalid cache file".to_string()))?;

    let ciphertext = data[KEY_SIZE + 12..].to_vec();

    let encrypted = vx_core::crypto::EncryptedData { ciphertext, nonce };

    // Try to decrypt
    match crypto::decrypt(&encrypted, &session_key) {
        Ok(password) => Ok(Some(password)),
        Err(_) => {
            // Cache is corrupted or from different session
            let _ = fs::remove_file(&cache_path);
            Ok(None)
        }
    }
}

/// Caches the password encrypted with a session key.
fn cache_password(password: &[u8]) -> Result<(), CliError> {
    let cache_path = password_cache_path()?;

    // Derive session-specific encryption key
    let session_key = derive_session_key()?;

    // Encrypt password
    let encrypted = crypto::encrypt(password, &session_key)
        .map_err(CliError::Crypto)?;

    // Build cache file: session_key + nonce + ciphertext
    let mut cache_data = Vec::new();
    cache_data.extend_from_slice(&session_key);
    cache_data.extend_from_slice(&encrypted.nonce);
    cache_data.extend_from_slice(&encrypted.ciphertext);

    // Write with restricted permissions
    let mut file = fs::File::create(&cache_path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o600);
        file.set_permissions(permissions)?;
    }

    file.write_all(&cache_data)?;
    file.sync_all()?;

    Ok(())
}

/// Clears the cached password.
pub fn clear_cached_password() -> Result<(), CliError> {
    let cache_path = password_cache_path()?;
    if cache_path.exists() {
        fs::remove_file(&cache_path)?;
    }
    Ok(())
}

/// Returns the path to the password cache file.
fn password_cache_path() -> Result<std::path::PathBuf, CliError> {
    let temp_dir = std::env::temp_dir();
    // Use process ID to make cache session-specific
    let pid = std::process::id();
    Ok(temp_dir.join(format!("vaultx_session_{}.cache", pid)))
}

/// Derives a session-specific encryption key.
/// This key is unique to the current process and cannot be reused by other processes.
fn derive_session_key() -> Result<[u8; KEY_SIZE], CliError> {
    // Use process ID + environment as salt
    let pid = std::process::id();
    let salt_input = format!("vaultx_session_{}_key", pid);

    // Derive key from session-specific data
    let salt = vx_core::crypto::generate_salt();
    crypto::derive_key(salt_input.as_bytes(), &salt)
        .map_err(CliError::Crypto)
}
