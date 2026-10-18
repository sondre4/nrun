use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

use crate::cli::handlers;

// TODO: include command examples
#[derive(Parser)]
#[command(
    name = "nrun",
    version,
    about = "Nrun is a tool designed for browsing and calling the Node scripts in projects and monorepos",
    long_about = "Nrun searches for a project/monorepo root, reads the project's package manager's lockfile, and looks for scripts defined in package.json files.\nCurrently, Nrun can recognize projects with npm, pnpm, bun, and yarn (newer) as the package manager.",
    after_help = "For more information about Nrun, see the README.md in the repository"
)]
pub struct Cli {
    /// Calling 'nrun' without a command/subcommand launches the TUI (Default behavior is to launch interactive TUI)
    #[command(subcommand)]
    pub command: Option<Command>,

    /// You can specify a package and script name to call that script directly using the project's package manager
    #[arg(short, long, requires = "script")]
    package: Option<String>,

    /// You can specify a package and script name to call that script directly using the project's package manager
    #[arg(short, long, requires = "package")]
    script: Option<String>,

    // default command arguments
    /// Optional: specify a path to a project/monorepo root directory
    #[arg(short, long, global = true)]
    workspace: Option<PathBuf>,

    /// Optional: specify how far out of the starting directory nrun should search for a project/monorepo root
    #[arg(short, long, global = true, default_value_t = 7)] // default depth is 7
    depth: u32,
}

#[derive(Subcommand)]
pub enum Command {
    /// Launch nrun's TUI. The TUI is also launched when no command/subcommand is specified (default)
    Tui,

    /// Search the packages and the scripts that Nrun has found from the CLI
    Search {
        /// The text to search for. Fuzzy-matches the search query in package names and script names/commands
        #[arg(short, long)]
        query: String,

        /// When displaying results directory to stdout, the output will be in JSON format
        #[arg(short, long, default_value_t = false)]
        json: bool,
        // /// Enable scoring/filtering debug output
        // #[arg(long, default_value_t = false)]
        // debug: bool,
    },

    /// List all packages and their scripts in a workspace
    #[command(visible_alias = "l", visible_alias = "ls")]
    List {
        /// Output the found scripts to stdout in JSON format
        #[arg(short, long, default_value_t = false)]
        json: bool,
    },
}

pub struct GlobalArguments {
    pub package: Option<String>,
    pub script: Option<String>,
    pub workspace: Option<PathBuf>,
    pub depth: u32,
}

impl GlobalArguments {
    pub fn from(input: &Cli) -> GlobalArguments {
        return GlobalArguments {
            package: input.package.clone(),
            script: input.script.clone(),
            workspace: input.workspace.clone(),
            depth: input.depth,
        };
    }
}

pub fn handle_command(input: Cli) -> () {
    let global_args = GlobalArguments::from(&input);

    match input.command {
        Some(Command::List {
            json,
        }) => {
            handlers::handle_list_packages(global_args, json);
        }

        Some(Command::Tui) => {
            handlers::handle_cli_default(global_args);
        }

        Some(Command::Search {
            query,
            json,
        }) => {
            handlers::handle_cli_search(global_args, &query, false, json);
        }

        // Default
        None => {
            handlers::handle_cli_default(global_args);
        }
    }
}
