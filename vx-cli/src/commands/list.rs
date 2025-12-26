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

    let has_projects = !vault.projects.is_empty();
    let has_ssh = !vault.ssh_identities.is_empty();
    let has_servers = !vault.ssh_servers.is_empty();

    if !has_projects && !has_ssh && !has_servers {
        println!("Vault is empty.");
        return Ok(());
    }

    // Show projects
    if has_projects {
        println!("Projects:");
        for (name, project) in &vault.projects {
            let secret_count = project.secrets.len();
            let secret_word = if secret_count == 1 { "secret" } else { "secrets" };
            println!("  • {} ({} {})", name, secret_count, secret_word);
        }
        println!();
    }

    // Show SSH servers (includes identities since they're 1:1 mapped)
    if has_servers {
        println!("SSH Servers:");
        for (name, server) in &vault.ssh_servers {
            println!("  • {} → {}@{}",
                name, server.username, server.ip_address);
        }
        println!();
    }

    // Show standalone SSH identities (not linked to servers)
    if has_ssh {
        let standalone_identities: Vec<_> = vault.ssh_identities.keys()
            .filter(|name| !vault.ssh_servers.contains_key(*name))
            .collect();

        if !standalone_identities.is_empty() {
            println!("SSH Identities (not yet configured as servers):");
            for name in standalone_identities {
                println!("  • {} (run: vx ssh connect {})", name, name);
            }
        }
    }

    Ok(())
}
