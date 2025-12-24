use crate::error::CliError;
use crate::input;
use crate::session;
use crate::storage;

pub fn execute(project: &str, key: Option<&str>) -> Result<(), CliError> {
    // Load vault with encryption key
    let (mut vault, password_bytes) = if let Some(cached) = session::get_cached_password()? {
        match storage::load_vault_with_key(&cached) {
            Ok((v, _)) => (v, cached),
            Err(_) => {
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
    };

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
    storage::save_vault(&vault, &password_bytes)?;

    Ok(())
}
