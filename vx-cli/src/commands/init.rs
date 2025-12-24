//! Initialize a new project in the vault.

use crate::error::CliError;
use crate::input;
use crate::session;
use crate::storage;

/// Executes the init command.
pub fn execute(project: &str) -> Result<(), CliError> {
    let (mut vault, password_bytes) = if storage::vault_exists()? {
        // Load existing vault
        // Check cache first
        if let Some(cached) = session::get_cached_password()? {
            match storage::load_vault_with_key(&cached) {
                Ok((v, _)) => (v, cached),
                Err(_) => {
                    // Cache invalid/stale
                    let _ = session::clear_cached_password();
                    let p = input::read_password("Enter master password: ")?;
                    let (v, _) = storage::load_vault_with_key(p.as_bytes())?;
                    (v, p.into_bytes())
                }
            }
        } else {
             let p = input::read_password("Enter master password: ")?;
             let (v, _) = storage::load_vault_with_key(p.as_bytes())?;
             (v, p.into_bytes())
        }
    } else {
        // Create new vault
        println!("Creating new vault...");
        let password = input::read_new_password()?;
        let (vault, _key) = storage::create_vault(password.as_bytes())?;
        (vault, password.into_bytes())
    };

    // Initialize the project
    vault.init_project(project)?;

    // Save the vault
    storage::save_vault(&vault, &password_bytes)?;

    println!("Project '{}' initialized successfully.", project);
    Ok(())
}