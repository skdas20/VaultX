use crate::error::CliError;
use crate::input;
use crate::storage;

pub fn execute(project: &str, key: &str) -> Result<(), CliError> {
    // Load vault with encryption key
    let password = input::read_password("Enter master password: ")?;
    let (mut vault, encryption_key) = storage::load_vault_with_key(password.as_bytes())?;

    // Check if secret exists
    let exists = vault
        .projects
        .get(project)
        .map(|p| p.secrets.contains_key(key))
        .unwrap_or(false);

    if !exists {
        return Err(CliError::Vault(vx_core::VaultError::SecretNotFound(key.to_string())));
    }

    println!("Editing secret '{}' in project '{}'.", key, project);
    
    // Read new secret value
    let secret_value = input::read_secret(None, None)?;

    // We preserve the existing TTL if we could read it, but currently the API doesn't easily expose getting just the TTL without full decryption. 
    // For now, "edit" will just update the value and reset/clear TTL unless we want to complicate the API. 
    // Usually "edit value" implies keeping metadata, but let's just update the value for simplicity as per request.
    // If the user wants to keep TTL, they might need to re-supply it, or we assume None (no expiry) or we fetch the old one.
    // Let's fetch the old one to be nice.
    
    // To get the old TTL, we'd need to inspect the secret struct. 
    // Currently `vault.projects` is public, so we can access it.
    let old_ttl_expiry = vault.projects.get(project).unwrap().secrets.get(key).unwrap().expires_at;
    
    // Calculate remaining TTL or just keep the absolute expiry? 
    // `add_secret` takes `ttl_seconds`. If we pass None, it sets expires_at to None.
    // If we want to preserve the *remaining* time, we need to calculate it.
    // Or we just accept that editing resets TTL or removes it. 
    // Let's keep it simple: Editing updates the value. If they want TTL, they use `add` with `--ttl`.
    // Actually, if I use `add_secret` with None, it clears the expiry.
    // Let's stick to simple update: Update value, clear TTL (or maybe I should preserve it? The user didn't specify).
    // I'll preserve it if possible.
    
    let ttl_seconds = if let Some(expiry) = old_ttl_expiry {
        let now = vx_core::ttl::current_timestamp();
        if expiry > now {
            Some(expiry - now)
        } else {
            None
        }
    } else {
        None
    };

    // Update secret
    vault.add_secret(project, key, &secret_value, &encryption_key, ttl_seconds)?;

    // Save vault
    storage::save_vault(&vault, password.as_bytes())?;

    println!("Secret '{}' updated.", key);

    Ok(())
}