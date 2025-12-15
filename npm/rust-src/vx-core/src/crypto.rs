//! Cryptographic operations for VaultX.
//!
//! This module provides:
//! - Key derivation using Argon2id
//! - Encryption/decryption using AES-256-GCM
//!
//! # Security Notes
//! - Argon2id is used with 64MB memory cost and 3 iterations
//! - Each encryption uses a unique random 96-bit nonce
//! - Nonces are stored alongside ciphertext

use crate::error::CryptoError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::RngCore;

/// Size of the encryption key in bytes (256 bits)
pub const KEY_SIZE: usize = 32;

/// Size of the nonce in bytes (96 bits)
pub const NONCE_SIZE: usize = 12;

/// Size of the salt in bytes
pub const SALT_SIZE: usize = 32;

/// Argon2 memory cost in KiB (64 MB)
const ARGON2_MEMORY_COST: u32 = 65536;

/// Argon2 iteration count
const ARGON2_ITERATIONS: u32 = 3;

/// Argon2 parallelism
const ARGON2_PARALLELISM: u32 = 4;

/// Encrypted data containing ciphertext and nonce.
#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; NONCE_SIZE],
}

/// Generates a random salt for key derivation.
pub fn generate_salt() -> [u8; SALT_SIZE] {
    let mut salt = [0u8; SALT_SIZE];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

/// Generates a random nonce for encryption.
fn generate_nonce() -> [u8; NONCE_SIZE] {
    let mut nonce = [0u8; NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

/// Derives an encryption key from a password using Argon2id.
///
/// # Arguments
/// * `password` - The user's password
/// * `salt` - A random salt (should be stored with the vault)
///
/// # Security
/// Uses Argon2id with:
/// - 64 MB memory cost
/// - 3 iterations
/// - 4 parallelism lanes
pub fn derive_key(password: &[u8], salt: &[u8]) -> Result<[u8; KEY_SIZE], CryptoError> {
    let params = Params::new(
        ARGON2_MEMORY_COST,
        ARGON2_ITERATIONS,
        ARGON2_PARALLELISM,
        Some(KEY_SIZE),
    )
    .map_err(|_| CryptoError::KeyDerivationFailed)?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = [0u8; KEY_SIZE];
    argon2
        .hash_password_into(password, salt, &mut key)
        .map_err(|_| CryptoError::KeyDerivationFailed)?;

    Ok(key)
}

/// Encrypts plaintext using AES-256-GCM.
///
/// # Arguments
/// * `plaintext` - Data to encrypt
/// * `key` - 256-bit encryption key
///
/// # Returns
/// Encrypted data containing ciphertext and nonce
///
/// # Security
/// - Uses a unique random nonce for each encryption
/// - Nonce must be stored with ciphertext for decryption
pub fn encrypt(plaintext: &[u8], key: &[u8; KEY_SIZE]) -> Result<EncryptedData, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidKeyLength)?;

    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| CryptoError::EncryptionFailed)?;

    Ok(EncryptedData {
        ciphertext,
        nonce: nonce_bytes,
    })
}

/// Decrypts ciphertext using AES-256-GCM.
///
/// # Arguments
/// * `encrypted` - Encrypted data with nonce
/// * `key` - 256-bit encryption key
///
/// # Returns
/// Decrypted plaintext
///
/// # Security
/// - Verifies authentication tag before returning plaintext
/// - Returns generic error on failure (prevents oracle attacks)
pub fn decrypt(encrypted: &EncryptedData, key: &[u8; KEY_SIZE]) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidKeyLength)?;

    let nonce = Nonce::from_slice(&encrypted.nonce);

    cipher
        .decrypt(nonce, encrypted.ciphertext.as_ref())
        .map_err(|_| CryptoError::DecryptionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let password = b"test_password";
        let salt = generate_salt();

        let key = derive_key(password, &salt).unwrap();
        assert_eq!(key.len(), KEY_SIZE);

        // Same password and salt should produce same key
        let key2 = derive_key(password, &salt).unwrap();
        assert_eq!(key, key2);
    }

    #[test]
    fn test_different_salts_produce_different_keys() {
        let password = b"test_password";
        let salt1 = generate_salt();
        let salt2 = generate_salt();

        let key1 = derive_key(password, &salt1).unwrap();
        let key2 = derive_key(password, &salt2).unwrap();

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let password = b"test_password";
        let salt = generate_salt();
        let key = derive_key(password, &salt).unwrap();

        let plaintext = b"Hello, VaultX!";
        let encrypted = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_wrong_key_fails_decryption() {
        let salt = generate_salt();
        let key1 = derive_key(b"password1", &salt).unwrap();
        let key2 = derive_key(b"password2", &salt).unwrap();

        let plaintext = b"Secret data";
        let encrypted = encrypt(plaintext, &key1).unwrap();

        let result = decrypt(&encrypted, &key2);
        assert!(result.is_err());
    }

    #[test]
    fn test_unique_nonces() {
        let key = [0u8; KEY_SIZE];
        let plaintext = b"test";

        let encrypted1 = encrypt(plaintext, &key).unwrap();
        let encrypted2 = encrypt(plaintext, &key).unwrap();

        assert_ne!(encrypted1.nonce, encrypted2.nonce);
    }
}
