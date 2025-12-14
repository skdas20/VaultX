//! Get a secret from a project.

use crate::error::CliError;
use crate::input;
use crate::storage;

/// Executes the get command.
pub fn execute(project: &str, key: &str) -> Result<(), CliError> {
    // Load vault with encryption key
    let password = input::read_password("Enter master password: ")?;
    let (vault, encryption_key) = storage::load_vault_with_key(password.as_bytes())?;

    // Get secret
    let secret_value = vault.get_secret(project, key, &encryption_key)?;

    // Output to stdout
    // Note: We output raw bytes to handle binary secrets
    use std::io::{self, Write};
    io::stdout().write_all(&secret_value)?;
    io::stdout().flush()?;

    // Add newline if output is text
    if secret_value.iter().all(|&b| b != 0 && (b.is_ascii() || b > 127)) {
        println!();
    }

    Ok(())
}
