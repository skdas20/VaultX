//! VaultX CLI - A zero-trust developer vault
//!
//! This is the main entry point for the `vx` command-line tool.

mod commands;
mod error;
mod input;
mod storage;

use clap::{Parser, Subcommand};
use error::CliError;

const BANNER: &str = "\x1b[36m
__      __          _ _  __   __
\\ \\    / /         | | |\\ \\ / /
 \\ \\  / /_ _ _   _| | |_ \\ V /
  \\ \\/ / _` | | | | | __| > <
   \\  / (_| | |_| | | |_ / . \\
    \\/ \\__,_|\\__,_|_|\\__/_/ \\_\\
\x1b[0m
   \x1b[1mVaultX\x1b[0m - Secure Secrets Management
   Created by \x1b[36mSumit Kumar Das\x1b[0m
";

#[derive(Parser)]
#[command(name = "vx")]
#[command(author, version, about = "VaultX - A zero-trust developer vault")]
#[command(propagate_version = true)]
#[command(before_help = BANNER)]
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

        /// Secret key name (optional for interactive mode)
        key: Option<String>,

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

    /// Get a secret from a project (or all secrets if no key specified)
    Get {
        /// Project name
        project: String,

        /// Secret key name (optional - omit to see all secrets)
        key: Option<String>,
    },

    /// List all projects in the vault
    List,

    /// List all secrets in a project
    Secrets {
        /// Project name
        project: String,
    },

    /// Audit the vault for security issues
    Audit,

    /// SSH identity management
    Ssh {
        #[command(subcommand)]
        command: SshCommands,
    },

    /// Remove a secret or project from the vault
    Remove {
        /// Project name
        project: String,

        /// The name of the secret to remove (optional - if omitted, removes the entire project)
        key: Option<String>,
    },

    /// Edit a secret in the vault
    Edit {
        /// Project name
        project: String,

        /// The name of the secret to edit
        key: String,
    },

    /// Update the VX CLI to the latest version
    Update {
        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Cache vault password for current session
    Login,
}

#[derive(Subcommand)]
enum SshCommands {
    /// Initialize a new SSH identity
    Init {
        /// Name for the SSH identity
        name: String,
    },

    /// Connect to a server using an SSH identity or configure a server
    Connect {
        /// SSH identity or server name
        identity_or_server: String,

        /// Target in user@host format (optional if server configured)
        target: Option<String>,

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
        } => commands::add::execute(&project, key.as_deref(), file, env, ttl),
        Commands::Get { project, key } => commands::get::execute(&project, key.as_deref()),
        Commands::List => commands::list::execute(),
        Commands::Secrets { project } => commands::list_secrets::execute(&project),
        Commands::Audit => commands::audit::execute(),
        Commands::Ssh { command } => match command {
            SshCommands::Init { name } => commands::ssh::init(&name),
            SshCommands::Connect {
                identity_or_server,
                target,
                args,
            } => commands::ssh::connect_dispatch(&identity_or_server, target.as_deref(), &args),
        },
        Commands::Remove { project, key } => commands::remove::execute(&project, key.as_deref()),
        Commands::Edit { project, key } => commands::edit::execute(&project, &key),
        Commands::Update { yes } => commands::update::execute(yes),
        Commands::Login => commands::login::execute(),
    }
}
