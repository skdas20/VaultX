use crate::error::CliError;
use crate::input;
use crate::session;
use crate::storage;

pub fn execute(project: &str, key: &str) -> Result<(), CliError> {
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

    // Preserve existing TTL
    let old_ttl_expiry = vault.projects.get(project).unwrap().secrets.get(key).unwrap().expires_at;
    
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
    storage::save_vault(&vault, &password_bytes)?;

    println!("Secret '{}' updated.", key);

    Ok(())
}
