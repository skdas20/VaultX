//! VaultX CLI - A zero-trust developer vault
//!
//! This is the main entry point for the `vx` command-line tool.

mod commands;
mod error;
mod input;
mod storage;

use clap::{Parser, Subcommand};
use error::CliError;

#[derive(Parser)]
#[command(name = "vx")]
#[command(author, version, about = "VaultX - A zero-trust developer vault")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project in the vault
    Init {
        /// Name of the project to create
        project: String,
    },

    /// Add a secret to a project
    Add {
        /// Project name
        project: String,

        /// Secret key name
        key: String,

        /// Read secret from file
        #[arg(long, value_name = "FILE")]
        file: Option<String>,

        /// Read secret from environment variable
        #[arg(long, value_name = "VAR")]
        env: Option<String>,

        /// Time-to-live (e.g., 6h, 7d, 2w)
        #[arg(long)]
        ttl: Option<String>,
    },

    /// Get a secret from a project
    Get {
        /// Project name
        project: String,

        /// Secret key name
        key: String,
    },

    /// Audit the vault for security issues
    Audit,

    /// SSH identity management
    Ssh {
        #[command(subcommand)]
        command: SshCommands,
    },
}

#[derive(Subcommand)]
enum SshCommands {
    /// Initialize a new SSH identity
    Init {
        /// Name for the SSH identity
        name: String,
    },

    /// Connect to a server using an SSH identity
    Connect {
        /// SSH identity name
        identity: String,

        /// Target in user@host format
        target: String,

        /// Additional SSH arguments
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { project } => commands::init::execute(&project),
        Commands::Add {
            project,
            key,
            file,
            env,
            ttl,
        } => commands::add::execute(&project, &key, file, env, ttl),
        Commands::Get { project, key } => commands::get::execute(&project, &key),
        Commands::Audit => commands::audit::execute(),
        Commands::Ssh { command } => match command {
            SshCommands::Init { name } => commands::ssh::init(&name),
            SshCommands::Connect {
                identity,
                target,
                args,
            } => commands::ssh::connect(&identity, &target, &args),
        },
    }
}
