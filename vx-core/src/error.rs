//! Error types for VaultX core operations.
//!
//! # Security Note
//! Error messages must NEVER contain secret values.
//! Authentication errors should be generic to prevent oracle attacks.

use thiserror::Error;

/// Errors that can occur during cryptographic operations.
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Key derivation failed")]
    KeyDerivationFailed,

    #[error("Encryption failed")]
    EncryptionFailed,

    #[error("Decryption failed: authentication error")]
    DecryptionFailed,

    #[error("Invalid nonce length")]
    InvalidNonce,

    #[error("Invalid key length")]
    InvalidKeyLength,
}

/// Errors that can occur during vault operations.
#[derive(Debug, Error)]
pub enum VaultError {
    #[error("Project '{0}' not found")]
    ProjectNotFound(String),

    #[error("Secret '{0}' not found")]
    SecretNotFound(String),

    #[error("Project '{0}' already exists")]
    ProjectAlreadyExists(String),

    #[error("Secret '{0}' has expired")]
    SecretExpired(String),

    #[error("SSH identity '{0}' not found")]
    IdentityNotFound(String),

    #[error("SSH identity '{0}' already exists")]
    IdentityAlreadyExists(String),

    #[error("SSH server '{0}' not found")]
    ServerNotFound(String),

    #[error("SSH server '{0}' already exists")]
    ServerAlreadyExists(String),

    #[error("Invalid IP address format: '{0}'")]
    InvalidIpAddress(String),

    #[error("Vault file is corrupted or has been tampered with")]
    CorruptedVault,

    #[error("Invalid password or corrupted vault")]
    AuthenticationFailed,

    #[error("Invalid vault format: {0}")]
    InvalidFormat(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Cryptographic error: {0}")]
    CryptoError(#[from] CryptoError),
}

/// Errors that can occur during TTL parsing.
#[derive(Debug, Error)]
pub enum TtlError {
    #[error("Invalid TTL format: '{0}'")]
    InvalidFormat(String),

    #[error("Invalid time unit: '{0}'. Use m (minutes), h (hours), d (days), or w (weeks)")]
    InvalidUnit(char),

    #[error("TTL value overflow")]
    Overflow,

    #[error("TTL value must be positive")]
    ZeroOrNegative,
}

/// Errors that can occur during SSH operations.
#[derive(Debug, Error)]
pub enum SshError {
    #[error("Failed to generate SSH keypair")]
    KeyGenerationFailed,

    #[error("Invalid SSH key format")]
    InvalidKeyFormat,

    #[error("SSH key encryption failed")]
    EncryptionFailed,

    #[error("SSH key decryption failed")]
    DecryptionFailed,
}
