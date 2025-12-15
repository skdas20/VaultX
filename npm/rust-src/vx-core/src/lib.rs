//! VaultX Core Library
//!
//! This crate contains all cryptographic operations for VaultX.
//! It is designed to be compiled to WebAssembly for cross-runtime portability.
//!
//! # Security Note
//! All cryptographic operations are isolated in this crate.
//! The CLI layer should never perform crypto operations directly.

pub mod crypto;
pub mod error;
pub mod ssh;
pub mod ttl;
pub mod vault;

// Re-export main types for convenience
pub use crypto::KEY_SIZE;
pub use error::{CryptoError, SshError, TtlError, VaultError};
pub use vault::{Project, Secret, SshIdentity, Vault};

#[cfg(feature = "wasm")]
pub mod wasm;
