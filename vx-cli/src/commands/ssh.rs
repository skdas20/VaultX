//! SSH identity management commands.

use crate::error::CliError;
use crate::input;
use crate::storage;
use std::fs;
use std::io::Write;
use std::process::Command;
use vx_core::ssh;

/// Executes the ssh init command.
pub fn init(name: &str) -> Result<(), CliError> {
    // Load or create vault
    let (mut vault, encryption_key, password) = if storage::vault_exists()? {
        let password = input::read_password("Enter master password: ")?;
        let (vault, key) = storage::load_vault_with_key(password.as_bytes())?;
        (vault, key, password)
    } else {
        println!("Creating new vault...");
        let password = input::read_new_password()?;
        let (vault, key) = storage::create_vault(password.as_bytes())?;
        (vault, key, password)
    };

    // Generate keypair
    let (public_key, private_key) = ssh::generate_keypair().map_err(|e| {
        CliError::SshError(format!("Failed to generate keypair: {}", e))
    })?;

    // Store identity
    vault.add_ssh_identity(name, public_key.clone(), &private_key, &encryption_key)?;

    // Save vault
    storage::save_vault(&vault, password.as_bytes())?;

    // Display public key and setup commands
    println!("\nSSH identity '{}' created successfully.\n", name);
    println!("Public key:");
    println!("{}\n", public_key);
    println!("Setup commands for remote server:");
    println!("{}", ssh::generate_setup_commands(&public_key));

    Ok(())
}

/// Executes the ssh connect command.
///
/// # Security
/// - Decrypts private key in memory
/// - Writes to temp file with 0600 permissions
/// - Deletes temp file after SSH session
pub fn connect(identity: &str, target: &str, extra_args: &[String]) -> Result<(), CliError> {
    // Load vault with encryption key
    let password = input::read_password("Enter master password: ")?;
    let (vault, encryption_key) = storage::load_vault_with_key(password.as_bytes())?;

    // Get SSH identity
    let (_public_key, private_key_bytes) = vault.get_ssh_identity(identity, &encryption_key)?;

    // Reconstruct signing key and format private key
    let signing_key = ssh::reconstruct_signing_key(&private_key_bytes)
        .map_err(|e| CliError::SshError(format!("Invalid key format: {}", e)))?;

    let public_key_bytes = signing_key.verifying_key();
    let private_key_pem = ssh::format_private_key(&private_key_bytes, public_key_bytes.as_bytes())
        .map_err(|e| CliError::SshError(format!("Failed to format private key: {}", e)))?;

    // Create temp file for private key
    let temp_dir = tempfile::tempdir()?;
    let key_path = temp_dir.path().join("id_temp");

    // Write private key with restricted permissions
    {
        let mut file = fs::File::create(&key_path)?;

        // Set permissions to 0600 before writing (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = fs::Permissions::from_mode(0o600);
            file.set_permissions(permissions)?;
        }

        file.write_all(private_key_pem.as_bytes())?;
        file.sync_all()?;
    }

    // Build SSH command
    let mut cmd = Command::new("ssh");
    cmd.arg("-i").arg(&key_path);
    cmd.arg(target);

    // Add extra arguments
    for arg in extra_args {
        cmd.arg(arg);
    }

    println!("Connecting to {} using identity '{}'...\n", target, identity);

    // Execute SSH
    let status = cmd.status().map_err(|e| {
        CliError::SshError(format!("Failed to execute ssh: {}", e))
    })?;

    // Temp directory and key file are automatically cleaned up when temp_dir goes out of scope

    if !status.success() {
        return Err(CliError::SshError(format!(
            "SSH exited with status: {}",
            status.code().unwrap_or(-1)
        )));
    }

    Ok(())
}
