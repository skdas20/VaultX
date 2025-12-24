//! List all secrets in a project.

use crate::error::CliError;

use crate::storage;
use vx_core::ttl;

/// Executes the list-secrets command.
pub fn execute(project: &str) -> Result<(), CliError> {
    // Load vault with encryption key
    let (vault, _encryption_key) = storage::load_vault_with_key_auto()?;

    // Get project
    let proj = vault
        .projects
        .get(project)
        .ok_or_else(|| CliError::ProjectNotFound(project.to_string()))?;

    if proj.secrets.is_empty() {
        println!("No secrets in project '{}'.", project);
        return Ok(());
    }

    println!("Secrets in project '{}':  ", project);
    
    let now = ttl::current_timestamp();
    
    for (key, secret) in &proj.secrets {
        // Check if expired
        let status = if let Some(expires_at) = secret.expires_at {
            if expires_at < now {
                "(expired)".to_string()
            } else {
                let remaining = expires_at - now;
                let hours = remaining / 3600;
                let minutes = (remaining % 3600) / 60;
                if hours > 0 {
                    format!("(expires in {}h {}m)", hours, minutes)
                } else {
                    format!("(expires in {}m)", minutes)
                }
            }
        } else {
            "(no expiry)".to_string()
        };
        
        println!("  • {} {}", key, status);
    }

    Ok(())
}
