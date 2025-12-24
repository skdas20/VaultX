//! Add a secret to a project.

use crate::error::CliError;
use crate::input;
use crate::session;
use crate::storage;
use vx_core::{ttl, Vault, KEY_SIZE};

/// Executes the add command.
pub fn execute(
    project: &str,
    key: Option<&str>,
    file: Option<String>,
    env: Option<String>,
    ttl_str: Option<String>,
) -> Result<(), CliError> {
    // Load vault with encryption key
    let (mut vault, encryption_key, password_bytes) = if let Some(cached) = session::get_cached_password()? {
        match storage::load_vault_with_key(&cached) {
            Ok((v, k)) => (v, k, cached),
            Err(_) => {
                let _ = session::clear_cached_password();
                let p = input::read_password("Enter master password: ")?;
                let (v, k) = storage::load_vault_with_key(p.as_bytes())?;
                (v, k, p.into_bytes())
            }
        }
    } else {
         let p = input::read_password("Enter master password: ")?;
         let (v, k) = storage::load_vault_with_key(p.as_bytes())?;
         (v, k, p.into_bytes())
    };

    // Parse TTL if provided
    let ttl_seconds = if let Some(ttl) = ttl_str {
        Some(ttl::parse_ttl(&ttl).map_err(|e| CliError::InvalidTtl(e.to_string()))?)
    } else {
        None
    };

    if let Some(k) = key {
        // Single add mode
        add_secret_interactive(
            &mut vault,
            project,
            k,
            file,
            env,
            &encryption_key,
            ttl_seconds,
        )?;
    } else {
        // Interactive mode
        println!("Interactive mode enabled. Leave key empty to save and exit.");
        loop {
            let key_input = input::read_input("Enter key name: ")?;
            if key_input.is_empty() {
                break;
            }

            // In interactive loop, we don't support file/env args for each item, only interactive prompt
            match add_secret_interactive(
                &mut vault,
                project,
                &key_input,
                None,
                None,
                &encryption_key,
                ttl_seconds,
            ) {
                Ok(_) => {}
                Err(e) => eprintln!("Error adding secret: {}", e),
            }
        }
    }

    // Save vault
    storage::save_vault(&vault, &password_bytes)?;

    Ok(())
}

fn add_secret_interactive(
    vault: &mut Vault,
    project: &str,
    key: &str,
    file: Option<String>,
    env: Option<String>,
    encryption_key: &[u8; KEY_SIZE],
    ttl_seconds: Option<u64>,
) -> Result<(), CliError> {
    // Check if secret already exists
    if vault
        .projects
        .get(project)
        .map(|p| p.secrets.contains_key(key))
        .unwrap_or(false)
    {
        if !input::confirm(&format!("Secret '{}' already exists. Overwrite?", key))? {
            println!("Skipped.");
            return Ok(());
        }
    }

    // Read secret value
    let secret_value = input::read_secret(file.as_deref(), env.as_deref())?;

    // Add secret
    vault.add_secret(project, key, &secret_value, encryption_key, ttl_seconds)?;

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