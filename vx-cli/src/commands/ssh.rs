//! SSH identity and server management commands.

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
        let (vault, key) = storage::load_vault_with_key_auto()?;
        let password = get_password_for_save()?;
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
    println!("\n✓ SSH identity '{}' created successfully.\n", name);
    println!("Public key:");
    println!("{}\n", public_key);
    println!("Setup commands for remote server:");
    println!("{}", ssh::generate_setup_commands(&public_key));

    Ok(())
}

/// Dispatches SSH connect based on whether argument is identity or server.
pub fn connect_dispatch(
    identity_or_server: &str,
    target: Option<&str>,
    extra_args: &[String],
) -> Result<(), CliError> {
    // Load vault to check what we're dealing with
    let (vault, encryption_key) = storage::load_vault_with_key_auto()?;

    // Check if it's a configured server
    if vault.has_ssh_server(identity_or_server) {
        // It's a server name - use server shorthand
        connect_server(&vault, &encryption_key, identity_or_server, extra_args)
    } else if let Some(tgt) = target {
        // It's identity + target - use original connect logic
        connect_with_identity(&vault, &encryption_key, identity_or_server, tgt, extra_args)
    } else {
        // Check if it's "connect <servername>" for setup
        // This is the new interactive setup command
        setup_server(identity_or_server)
    }
}

/// Interactive setup for SSH server configuration.
fn setup_server(servername: &str) -> Result<(), CliError> {
    println!("Setting up SSH server configuration: {}", servername);

    // Load vault
    let (mut vault, _encryption_key) = storage::load_vault_with_key_auto()?;
    let password = get_password_for_save()?;

    // Check if server already exists
    if vault.has_ssh_server(servername) {
        if !input::confirm(&format!(
            "Server '{}' already configured. Overwrite?",
            servername
        ))? {
            println!("Cancelled.");
            return Ok(());
        }
    }

    // Check if identity with same name exists
    if !vault.ssh_identities.contains_key(servername) {
        return Err(CliError::SshError(format!(
            "SSH identity '{}' not found. Create it first with: vx ssh init {}",
            servername, servername
        )));
    }

    // Prompt for configuration
    let username = input::read_input("Remote username: ")?;
    if username.is_empty() {
        return Err(CliError::Generic("Username cannot be empty".to_string()));
    }

    let ip_address = input::read_input("Remote IP address: ")?;
    if ip_address.is_empty() {
        return Err(CliError::Generic("IP address cannot be empty".to_string()));
    }

    // Basic IP validation
    validate_ip_or_hostname(&ip_address)?;

    // Save configuration
    vault.add_ssh_server(
        servername,
        username.clone(),
        ip_address.clone(),
        servername.to_string(), // Identity has same name as server
    )?;

    storage::save_vault(&vault, password.as_bytes())?;

    println!(
        "\n✓ Server '{}' configured successfully!",
        servername
    );
    println!("  Username: {}", username);
    println!("  IP: {}", ip_address);
    println!("  Identity: {}", servername);
    println!("\nConnect with: vx ssh connect {}", servername);

    Ok(())
}

/// Connects using a configured server shorthand.
fn connect_server(
    vault: &vx_core::Vault,
    encryption_key: &[u8; 32],
    servername: &str,
    command_args: &[String],
) -> Result<(), CliError> {
    // Get server config
    let server = vault
        .get_ssh_server(servername)
        .map_err(|_| CliError::SshError(format!("Server '{}' not found", servername)))?;

    // Get SSH identity
    let (_public_key, private_key_bytes) =
        vault.get_ssh_identity(&server.identity_name, encryption_key)?;

    // Build target string
    let target = format!("{}@{}", server.username, server.ip_address);

    // Use existing connection logic
    execute_ssh_connection(&private_key_bytes, &target, &server.identity_name, command_args)
}

/// Connects using identity and target (original behavior).
fn connect_with_identity(
    vault: &vx_core::Vault,
    encryption_key: &[u8; 32],
    identity: &str,
    target: &str,
    extra_args: &[String],
) -> Result<(), CliError> {
    // Get SSH identity
    let (_public_key, private_key_bytes) = vault.get_ssh_identity(identity, encryption_key)?;

    execute_ssh_connection(&private_key_bytes, target, identity, extra_args)
}

/// Common SSH connection execution logic.
///
/// # Security
/// - Decrypts private key in memory
/// - Writes to temp file with 0600 permissions
/// - Deletes temp file after SSH session
fn execute_ssh_connection(
    private_key_bytes: &[u8],
    target: &str,
    identity_name: &str,
    extra_args: &[String],
) -> Result<(), CliError> {
    // Reconstruct signing key and format private key
    let signing_key = ssh::reconstruct_signing_key(private_key_bytes)
        .map_err(|e| CliError::SshError(format!("Invalid key format: {}", e)))?;

    let public_key_bytes = signing_key.verifying_key();
    let private_key_pem = ssh::format_private_key(private_key_bytes, public_key_bytes.as_bytes())
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

    // Build SSH command
    let mut cmd = Command::new("ssh");
    cmd.arg("-i").arg(&key_path);
    cmd.arg(target);

    // Add extra arguments
    for arg in extra_args {
        cmd.arg(arg);
    }

    let message = if extra_args.is_empty() {
        format!(
            "Connecting to {} using identity '{}'...\n",
            target, identity_name
        )
    } else {
        format!(
            "Executing command on {} using identity '{}'...\n",
            target, identity_name
        )
    };
    println!("{}", message);

    // Execute SSH with inherited stdio for interactive shell and command output
    let status = cmd
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .map_err(|e| CliError::SshError(format!("Failed to execute ssh: {}", e)))?;

    if !status.success() {
        return Err(CliError::SshError(format!(
            "SSH exited with status: {}",
            status.code().unwrap_or(-1)
        )));
    }

    Ok(())
}

/// Basic IP address or hostname validation.
fn validate_ip_or_hostname(addr: &str) -> Result<(), CliError> {
    if addr.is_empty() {
        return Err(CliError::Generic("Address cannot be empty".to_string()));
    }

    // Allow hostnames, IPv4, and IPv6
    // Basic validation - just check for reasonable characters
    let valid = addr.chars().all(|c| {
        c.is_alphanumeric() || c == '.' || c == ':' || c == '-' || c == '_'
    });

    if !valid {
        return Err(CliError::Generic(format!(
            "Invalid IP address or hostname: {}",
            addr
        )));
    }

    Ok(())
}

/// Gets password for vault save operation (from cache or prompt).
fn get_password_for_save() -> Result<String, CliError> {
    use crate::commands::login;

    // Try to use cached password
    if let Some(cached) = login::get_cached_password()? {
        // Verify it's still valid by trying to save a test vault
        match String::from_utf8(cached) {
            Ok(pwd) => return Ok(pwd),
            Err(_) => {
                // Clear invalid cache
                let _ = login::clear_cached_password();
            }
        }
    }

    // Fall back to prompting
    input::read_password("Enter master password: ")
}
