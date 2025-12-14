//! Get a secret from a project.

use crate::error::CliError;
use crate::input;
use crate::storage;
use vx_core::ttl;

/// Executes the get command.
/// If key is provided, gets that specific secret.
/// If key is not provided, shows all secrets in the project.
pub fn execute(project: &str, key: Option<&str>) -> Result<(), CliError> {
    // Load vault with encryption key
    let password = input::read_password("Enter master password: ")?;
    let (vault, encryption_key) = storage::load_vault_with_key(password.as_bytes())?;

    // Get project
    let proj = vault
        .projects
        .get(project)
        .ok_or_else(|| CliError::ProjectNotFound(project.to_string()))?;

    // If no specific key, show all secrets
    if key.is_none() {
        if proj.secrets.is_empty() {
            println!("No secrets in project '{}'.", project);
            return Ok(());
        }

        println!("\n📋 All Secrets in Project '{}'\n", project);
        println!("{:<30} {:<40} {:<15}", "KEY", "VALUE", "EXPIRY");
        println!("{}", "─".repeat(85));

        let now = ttl::current_timestamp();

        for (secret_key, secret) in &proj.secrets {
            // Decrypt value
            match vault.get_secret(project, secret_key, &encryption_key) {
                Ok(value) => {
                    let value_str = String::from_utf8_lossy(&value);
                    let value_display = if value_str.len() > 37 {
                        format!("{}...", &value_str[..37])
                    } else {
                        value_str.to_string()
                    };

                    let expiry_str = if let Some(expires_at) = secret.expires_at {
                        if expires_at < now {
                            "EXPIRED".to_string()
                        } else {
                            let remaining = expires_at - now;
                            let hours = remaining / 3600;
                            let minutes = (remaining % 3600) / 60;
                            if hours > 0 {
                                format!("{}h {}m", hours, minutes)
                            } else {
                                format!("{}m", minutes)
                            }
                        }
                    } else {
                        "∞".to_string()
                    };

                    println!("{:<30} {:<40} {:<15}", secret_key, value_display, expiry_str);
                }
                Err(e) => {
                    eprintln!("⚠️  Secret '{}' decryption failed: {}", secret_key, e);
                    eprintln!("   This may indicate the vault was corrupted or password is different.");
                    println!("{:<30} {:<40} {:<15}", secret_key, "[DECRYPTION FAILED]", "—");
                }
            }
        }
        println!("{}", "─".repeat(85));
        return Ok(());
    }

    // Get specific secret
    let key = key.unwrap();
    let secret_value = vault.get_secret(project, key, &encryption_key)?;

    // Output to stdout
    use std::io::{self, Write};
    io::stdout().write_all(&secret_value)?;
    io::stdout().flush()?;

    // Add newline if output is text
    if secret_value.iter().all(|&b| b != 0 && (b.is_ascii() || b > 127)) {
        println!();
    }

    Ok(())
}
