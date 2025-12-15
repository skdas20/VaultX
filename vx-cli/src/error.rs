//! CLI error types.

use thiserror::Error;
use vx_core::{CryptoError, VaultError};

#[derive(Debug, Error)]
pub enum CliError {
    #[error("{0}")]
    Vault(#[from] VaultError),

    #[error("{0}")]
    Crypto(#[from] CryptoError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to read password")]
    PasswordReadError,

    #[error("Passwords do not match")]
    PasswordMismatch,

    #[error("Secret input cancelled")]
    InputCancelled,

    #[error("Environment variable '{0}' not found")]
    EnvVarNotFound(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid TTL format: {0}")]
    InvalidTtl(String),

    #[error("Vault not initialized. Run 'vx init <project>' first.")]
    VaultNotFound,

    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("SSH connection failed: {0}")]
    SshError(String),

    #[error("Security violation: {0}")]
    SecurityViolation(String),

    #[error("Update failed: {0}")]
    UpdateError(String),

    #[error("{0}")]
    Generic(String),
}
