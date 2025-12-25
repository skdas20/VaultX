//! Secure copy (SCP) command implementation.

use crate::error::CliError;
use crate::storage;
use std::fs;
use std::io::Write;
use std::process::Command;
use vx_core::ssh;

/// Executes the scp command.
pub fn execute(server_name: &str, args: &[String]) -> Result<(), CliError> {
    // Load vault with encryption key (auto-cached)
    let (vault, encryption_key) = storage::load_vault_with_key_auto()?;

    // Get server config
    let server = vault
        .get_ssh_server(server_name)
        .map_err(|_| CliError::SshError(format!("Server '{}' not found", server_name)))?;

    // Get SSH identity
    let (_public_key, private_key_bytes) =
        vault.get_ssh_identity(&server.identity_name, &encryption_key)?;

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

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = fs::Permissions::from_mode(0o600);
            file.set_permissions(permissions)?;
        }

        file.write_all(private_key_pem.as_bytes())?;
        file.sync_all()?;
    }

    // Build SCP command
    let mut cmd = Command::new("scp");
    cmd.arg("-i").arg(&key_path);
    
    // Process arguments to replace ':' prefix with 'user@host:'
    for arg in args {
        if arg.starts_with(':') {
            // It's a remote path: :path/to/file -> user@host:path/to/file
            // or just : -> user@host:
            let path_part = &arg[1..];
            let remote_arg = if path_part.is_empty() {
                format!("{}@{}:", server.username, server.ip_address)
            } else {
                format!("{}@{}:{}", server.username, server.ip_address, path_part)
            };
            cmd.arg(remote_arg);
        } else {
            // Local path or option
            cmd.arg(arg);
        }
    }

    println!("Executing secure copy with identity '{}'...", server.identity_name);

    // Execute SCP
    let status = cmd
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .map_err(|e| CliError::SshError(format!("Failed to execute scp: {}", e)))?;

    if !status.success() {
        return Err(CliError::SshError(format!(
            "SCP exited with status: {}",
            status.code().unwrap_or(-1)
        )));
    }

    Ok(())
}
