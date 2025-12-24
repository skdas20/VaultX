//! List all projects in the vault.

use crate::error::CliError;

use crate::storage;

/// Executes the list command.
pub fn execute() -> Result<(), CliError> {
    // Check if vault exists
    if !storage::vault_exists()? {
        println!("No vault found. Run 'vx init <PROJECT>' to create one.");
        return Ok(());
    }

    // Load vault
    let (vault, _key) = storage::load_vault_with_key_auto()?;

    if vault.projects.is_empty() {
        println!("No projects in vault.");
        return Ok(());
    }

    println!("Projects in vault:");
    for (name, project) in &vault.projects {
        let secret_count = project.secrets.len();
        let secret_word = if secret_count == 1 { "secret" } else { "secrets" };
        println!("  • {} ({} {})", name, secret_count, secret_word);
    }

    Ok(())
}
