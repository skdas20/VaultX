//! Add a secret to a project.

use crate::error::CliError;
use crate::input;
use crate::storage;
use vx_core::ttl;

/// Executes the add command.
pub fn execute(
    project: &str,
    key: &str,
    file: Option<String>,
    env: Option<String>,
    ttl_str: Option<String>,
) -> Result<(), CliError> {
    // Load vault with encryption key
    let password = input::read_password("Enter master password: ")?;
    let (mut vault, encryption_key) = storage::load_vault_with_key(password.as_bytes())?;

    // Check if secret already exists
    if vault
        .projects
        .get(project)
        .map(|p| p.secrets.contains_key(key))
        .unwrap_or(false)
    {
        if !input::confirm(&format!("Secret '{}' already exists. Overwrite?", key))? {
            println!("Cancelled.");
            return Ok(());
        }
    }

    // Read secret value
    let secret_value = input::read_secret(file.as_deref(), env.as_deref())?;

    // Parse TTL if provided
    let ttl_seconds = if let Some(ttl) = ttl_str {
        Some(ttl::parse_ttl(&ttl).map_err(|e| CliError::InvalidTtl(e.to_string()))?)
    } else {
        None
    };

    // Add secret (using the vault's encryption key)
    vault.add_secret(project, key, &secret_value, &encryption_key, ttl_seconds)?;

    // Save vault
    storage::save_vault(&vault, password.as_bytes())?;

    if let Some(ttl) = ttl_seconds {
        println!(
            "Secret '{}' added to project '{}' (expires in {} seconds).",
            key, project, ttl
        );
    } else {
        println!("Secret '{}' added to project '{}'.", key, project);
    }

    Ok(())
}
