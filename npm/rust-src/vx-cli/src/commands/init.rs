//! Initialize a new project in the vault.

use crate::error::CliError;
use crate::input;
use crate::storage;

/// Executes the init command.
pub fn execute(project: &str) -> Result<(), CliError> {
    let (mut vault, password) = if storage::vault_exists()? {
        // Load existing vault
        let password = input::read_password("Enter master password: ")?;
        let (vault, _key) = storage::load_vault_with_key(password.as_bytes())?;
        (vault, password)
    } else {
        // Create new vault
        println!("Creating new vault...");
        let password = input::read_new_password()?;
        let (vault, _key) = storage::create_vault(password.as_bytes())?;
        (vault, password)
    };

    // Initialize the project
    vault.init_project(project)?;

    // Save the vault
    storage::save_vault(&vault, password.as_bytes())?;

    println!("Project '{}' initialized successfully.", project);
    Ok(())
}
