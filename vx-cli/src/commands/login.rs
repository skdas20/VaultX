//! Session password caching for VaultX.
//!
//! Caches the vault password in an encrypted temporary file to avoid
//! repeated password prompts within the same session.

use crate::error::CliError;
use crate::input;
use crate::session;
use crate::storage;

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
    session::cache_password(password.as_bytes())?;

    println!("✓ Password cached for current session.");
    println!("Subsequent commands will use cached password.");

    Ok(())
}