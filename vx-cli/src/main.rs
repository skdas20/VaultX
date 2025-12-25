//! VaultX CLI - A zero-trust developer vault
//!
//! This is the main entry point for the `vx` command-line tool.

mod commands;
mod error;
mod input;
mod session;
mod storage;

use clap::{Parser, Subcommand};
use error::CliError;

const BANNER: &str = r#"
__      __          _ _  __   __
\ \    / /         | | |\ \ / /
 \ \  / /_ _ _   _| | |_ \ V /
  \ \/ / _` | | | | | __| > <
   \  / (_| | |_| | | |_ / . \
    \/ \__,_|\__,_|_|\__/_/ \_\

   VaultX - Secure Secrets Management
   Created by Sumit Kumar Das
"#;

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
    ///
    /// Usage:
    ///   vx ssh init <name>           - Initialize new SSH identity
    ///   vx ssh <server>              - Connect to configured server
    ///   vx ssh <identity> <user@host> - Connect using identity
    Ssh {
        /// Subcommand (init, connect) or server/identity name
        #[arg(allow_hyphen_values = true)]
        target: Option<String>,

        /// Arguments for the command
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Secure copy to/from server
    ///
    /// Usage:
    ///   vx scp <server> <source> <dest>
    ///   Use ':' prefix to indicate remote path (e.g., :file.txt or :/tmp/file)
    Scp {
        /// Server name
        server: String,

        /// SCP arguments (use ':' prefix for remote paths)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
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
        Commands::Ssh { target, args } => commands::ssh::execute(target, args),
        Commands::Scp { server, args } => commands::scp::execute(&server, &args),
        Commands::Remove { project, key } => commands::remove::execute(&project, key.as_deref()),
        Commands::Edit { project, key } => commands::edit::execute(&project, &key),
        Commands::Update { yes } => commands::update::execute(yes),
        Commands::Login => commands::login::execute(),
    }
}