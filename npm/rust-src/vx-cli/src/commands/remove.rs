use crate::error::CliError;
use crate::input;
use crate::storage;

pub fn execute(project: &str, key: Option<&str>) -> Result<(), CliError> {
    // Load vault with encryption key
    let password = input::read_password("Enter master password: ")?;
    let (mut vault, _) = storage::load_vault_with_key(password.as_bytes())?;

    if let Some(k) = key {
        // Remove secret
        if !input::confirm(&format!("Are you sure you want to remove secret '{}' from project '{}'?", k, project))? {
            println!("Cancelled.");
            return Ok(());
        }
        vault.remove_secret(project, k)?;
        println!("Secret '{}' removed from project '{}'.", k, project);
    } else {
        // Remove project
        if !input::confirm(&format!("Are you sure you want to remove project '{}' and ALL its secrets?", project))? {
            println!("Cancelled.");
            return Ok(());
        }
        vault.remove_project(project)?;
        println!("Project '{}' removed.", project);
    }

    // Save vault
    storage::save_vault(&vault, password.as_bytes())?;

    Ok(())
}