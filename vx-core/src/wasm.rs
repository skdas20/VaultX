//! WebAssembly bindings for vx-core.
//!
//! These functions are exported via wasm-bindgen for use in other runtimes.

use wasm_bindgen::prelude::*;

use crate::crypto::{self, EncryptedData, KEY_SIZE, NONCE_SIZE, SALT_SIZE};
use crate::ssh;
use crate::ttl;
use crate::vault::{self, Vault};

/// Generates a random salt for key derivation.
#[wasm_bindgen]
pub fn wasm_generate_salt() -> Vec<u8> {
    crypto::generate_salt().to_vec()
}

/// Derives an encryption key from a password using Argon2id.
///
/// # Arguments
/// * `password` - The user's password as bytes
/// * `salt` - A 32-byte random salt
///
/// # Returns
/// A 32-byte encryption key
#[wasm_bindgen]
pub fn wasm_derive_key(password: &[u8], salt: &[u8]) -> Result<Vec<u8>, JsValue> {
    if salt.len() != SALT_SIZE {
        return Err(JsValue::from_str("Invalid salt length"));
    }

    crypto::derive_key(password, salt)
        .map(|k| k.to_vec())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Encrypts plaintext using AES-256-GCM.
///
/// # Arguments
/// * `plaintext` - Data to encrypt
/// * `key` - 32-byte encryption key
///
/// # Returns
/// Encrypted data as: nonce (12 bytes) || ciphertext
#[wasm_bindgen]
pub fn wasm_encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, JsValue> {
    if key.len() != KEY_SIZE {
        return Err(JsValue::from_str("Invalid key length"));
    }

    let key_array: [u8; KEY_SIZE] = key.try_into().unwrap();

    let encrypted = crypto::encrypt(plaintext, &key_array)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Return nonce || ciphertext
    let mut result = Vec::with_capacity(NONCE_SIZE + encrypted.ciphertext.len());
    result.extend_from_slice(&encrypted.nonce);
    result.extend_from_slice(&encrypted.ciphertext);

    Ok(result)
}

/// Decrypts ciphertext using AES-256-GCM.
///
/// # Arguments
/// * `encrypted` - Encrypted data as: nonce (12 bytes) || ciphertext
/// * `key` - 32-byte encryption key
///
/// # Returns
/// Decrypted plaintext
#[wasm_bindgen]
pub fn wasm_decrypt(encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>, JsValue> {
    if key.len() != KEY_SIZE {
        return Err(JsValue::from_str("Invalid key length"));
    }

    if encrypted.len() < NONCE_SIZE {
        return Err(JsValue::from_str("Invalid encrypted data"));
    }

    let key_array: [u8; KEY_SIZE] = key.try_into().unwrap();
    let nonce: [u8; NONCE_SIZE] = encrypted[..NONCE_SIZE].try_into().unwrap();
    let ciphertext = encrypted[NONCE_SIZE..].to_vec();

    let encrypted_data = EncryptedData { ciphertext, nonce };

    crypto::decrypt(&encrypted_data, &key_array)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Generates an ed25519 SSH keypair.
///
/// # Returns
/// JSON object with `public_key` (OpenSSH format) and `private_key` (bytes as array)
#[wasm_bindgen]
pub fn wasm_generate_ssh_keypair() -> Result<JsValue, JsValue> {
    let (public_key, private_key) = ssh::generate_keypair()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let result = serde_json::json!({
        "public_key": public_key,
        "private_key": private_key
    });

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parses a TTL string into seconds.
///
/// # Arguments
/// * `ttl` - TTL string (e.g., "6h", "7d", "2w")
///
/// # Returns
/// Duration in seconds
#[wasm_bindgen]
pub fn wasm_parse_ttl(ttl: &str) -> Result<u64, JsValue> {
    ttl::parse_ttl(ttl)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Checks if a timestamp has expired.
///
/// # Arguments
/// * `expires_at` - Expiration timestamp (0 for no expiration)
/// * `now` - Current timestamp
///
/// # Returns
/// true if expired, false otherwise
#[wasm_bindgen]
pub fn wasm_is_expired(expires_at: u64, now: u64) -> bool {
    if expires_at == 0 {
        false
    } else {
        ttl::is_expired(Some(expires_at), now)
    }
}

/// Creates a new empty vault and returns it as encrypted bytes.
///
/// # Arguments
/// * `password` - Master password
///
/// # Returns
/// Encrypted vault bytes
#[wasm_bindgen]
pub fn wasm_create_vault(password: &[u8]) -> Result<Vec<u8>, JsValue> {
    let vault = Vault::new();
    vault::save_vault(&vault, password)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Loads a vault from encrypted bytes.
///
/// # Arguments
/// * `data` - Encrypted vault bytes
/// * `password` - Master password
///
/// # Returns
/// JSON representation of the vault
#[wasm_bindgen]
pub fn wasm_load_vault(data: &[u8], password: &[u8]) -> Result<JsValue, JsValue> {
    let vault = vault::load_vault(data, password)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&vault)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Saves a vault to encrypted bytes.
///
/// # Arguments
/// * `vault_json` - JSON representation of the vault
/// * `password` - Master password
///
/// # Returns
/// Encrypted vault bytes
#[wasm_bindgen]
pub fn wasm_save_vault(vault_json: JsValue, password: &[u8]) -> Result<Vec<u8>, JsValue> {
    let vault: Vault = serde_wasm_bindgen::from_value(vault_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    vault::save_vault(&vault, password)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Returns the current Unix timestamp in seconds.
#[wasm_bindgen]
pub fn wasm_current_timestamp() -> u64 {
    ttl::current_timestamp()
}

/// Generates OS-specific SSH setup commands.
///
/// # Arguments
/// * `public_key` - OpenSSH formatted public key
///
/// # Returns
/// Shell commands for setting up the public key
#[wasm_bindgen]
pub fn wasm_generate_ssh_setup_commands(public_key: &str) -> String {
    ssh::generate_setup_commands(public_key)
}
