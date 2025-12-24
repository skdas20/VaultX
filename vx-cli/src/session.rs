//! Session management and password caching.

use crate::error::CliError;
use std::fs;
use std::io::Write;
use vx_core::crypto::{self, KEY_SIZE};

/// Gets the session identifier (Parent PID on Linux, PID elsewhere).
#[cfg(target_os = "linux")]
fn get_session_id() -> u32 {
    // On Linux, the parent process ID (shell) identifies the session
    if let Ok(stat) = std::fs::read_to_string("/proc/self/stat") {
        // Field 4 is PPID (1-based index 3)
        // Format: pid (comm) state ppid ...
        // We need to handle the fact that (comm) can contain spaces and parentheses
        // So simply splitting by whitespace might be dangerous if comm has spaces.
        // However, usually shell names don't have spaces.
        // A safer way is to find the last ')' and parse from there.
        
        if let Some(end_of_comm) = stat.rfind(')') {
            let after_comm = &stat[end_of_comm + 1..];
            if let Some(ppid_str) = after_comm.split_whitespace().nth(1) { // 0 is state, 1 is ppid
                 if let Ok(ppid) = ppid_str.parse::<u32>() {
                    return ppid;
                }
            }
        }
    }
    std::process::id()
}

#[cfg(not(target_os = "linux"))]
fn get_session_id() -> u32 {
    std::process::id()
}

/// Returns the path to the password cache file.
fn password_cache_path() -> Result<std::path::PathBuf, CliError> {
    let temp_dir = std::env::temp_dir();
    let session_id = get_session_id();
    Ok(temp_dir.join(format!("vaultx_session_{}.cache", session_id)))
}

/// Derives a session-specific encryption key.
fn derive_session_key() -> Result<[u8; KEY_SIZE], CliError> {
    let session_id = get_session_id();
    let salt_input = format!("vaultx_session_{}_key", session_id);

    // Create deterministic salt from session ID
    let mut salt = [0u8; 16];
    let sid_bytes = session_id.to_le_bytes();
    for i in 0..16 {
        salt[i] = sid_bytes[i % sid_bytes.len()].wrapping_add(i as u8);
    }

    crypto::derive_key(salt_input.as_bytes(), &salt)
        .map_err(CliError::Crypto)
}

/// Caches the password encrypted with a session key.
pub fn cache_password(password: &[u8]) -> Result<(), CliError> {
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

/// Gets the cached password if available and valid.
pub fn get_cached_password() -> Result<Option<Vec<u8>>, CliError> {
    let cache_path = password_cache_path()?;

    if !cache_path.exists() {
        return Ok(None);
    }

    // Read cache file
    let data = fs::read(&cache_path)?;

    if data.len() < KEY_SIZE + 12 {
        // Invalid cache file, remove it
        let _ = fs::remove_file(&cache_path);
        return Ok(None);
    }

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
            // Cache is corrupted or from different session (key mismatch)
            let _ = fs::remove_file(&cache_path);
            Ok(None)
        }
    }
}

/// Clears the cached password.
pub fn clear_cached_password() -> Result<(), CliError> {
    let cache_path = password_cache_path()?;
    if cache_path.exists() {
        fs::remove_file(&cache_path)?;
    }
    Ok(())
}
