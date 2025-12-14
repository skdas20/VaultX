//! Secure input handling for passwords and secrets.
//!
//! # Security Notes
//! - Passwords are read with terminal echo disabled
//! - Secrets can be provided via prompt, file, or environment variable
//! - Secrets are NEVER accepted as command-line arguments

use crate::error::CliError;
use std::io::{self, Write};

/// Prompts for a password with no echo.
pub fn read_password(prompt: &str) -> Result<String, CliError> {
    print!("{}", prompt);
    io::stdout().flush()?;

    rpassword::read_password().map_err(|_| CliError::PasswordReadError)
}

/// Prompts for a new password with confirmation.
pub fn read_new_password() -> Result<String, CliError> {
    let password = read_password("Enter master password: ")?;
    let confirm = read_password("Confirm master password: ")?;

    if password != confirm {
        return Err(CliError::PasswordMismatch);
    }

    Ok(password)
}

/// Reads a secret value from various sources.
///
/// # Arguments
/// * `file` - Optional file path to read from
/// * `env` - Optional environment variable name
///
/// # Returns
/// The secret value as bytes
///
/// # Security
/// If neither file nor env is provided, prompts for input with no echo.
pub fn read_secret(file: Option<&str>, env: Option<&str>) -> Result<Vec<u8>, CliError> {
    if let Some(file_path) = file {
        // Read from file
        std::fs::read(file_path).map_err(|_| CliError::FileNotFound(file_path.to_string()))
    } else if let Some(var_name) = env {
        // Read from environment variable
        std::env::var(var_name)
            .map(|v| v.into_bytes())
            .map_err(|_| CliError::EnvVarNotFound(var_name.to_string()))
    } else {
        // Prompt for input
        let secret = read_password("Enter secret value: ")?;
        Ok(secret.into_bytes())
    }
}

/// Prompts for confirmation.
pub fn confirm(prompt: &str) -> Result<bool, CliError> {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y") || input.trim().eq_ignore_ascii_case("yes"))
}
