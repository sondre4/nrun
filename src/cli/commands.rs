use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

use crate::cli::handlers;

#[derive(Parser)]
#[command(
    name = "nrun",
    version,
    about = "Calling 'nrun' without a command/subcommand launches the TUI (Default behavior is interactive)",
    long_about = "about2",
    after_help = "about3",
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Package name
    #[arg(short, long, requires = "script")]
    package: Option<String>,

    /// Script name
    #[arg(short, long, requires = "package")]
    script: Option<String>,

    // default command arguments
    /// Optional: specify a path to a workspace root
    #[arg(short, long, global = true)]
    workspace: Option<PathBuf>,

    /// Optional: specify how far out of the starting directory nrun should search for a workspace root
    #[arg(short, long, global = true, default_value_t = 7)] // default depth is 7
    depth: u32
}

#[derive(Subcommand)]
pub enum Command {
    /// Launch nrun's TUI. The TUI is also launched when no command/subcommand is specified (default)
    Tui,

    /// Search the data
    Search {
        /// The text to search for
        #[arg(short, long)]
        query: String,

        /// Output to stdout in JSON format
        #[arg(short, long, default_value_t = false)]
        json: bool,

        /// Enable scoring/filtering debug output
        #[arg(long, default_value_t = false)]
        debug: bool
    },

    /// List all packages and their scripts in a workspace
    #[command(visible_alias = "l", visible_alias = "ls")]
    List {

        /// Output to stdout in JSON format
        #[arg(short, long, default_value_t = false)]
        json: bool
    }
}

pub struct GlobalArguments {
    pub package: Option<String>,
    pub script: Option<String>,
    pub workspace: Option<PathBuf>,
    pub depth: u32
}

impl GlobalArguments {
    pub fn from(input: &Cli) -> GlobalArguments {
        return GlobalArguments {
            package: input.package.clone(),
            script: input.script.clone(),
            workspace: input.workspace.clone(),
            depth: input.depth.clone()
        };
    }
}

pub fn handle_command(input: Cli) -> () {
    let global_args = GlobalArguments::from(&input);

    match input.command {
        Some(Command::List { json }) => {
            handlers::handle_list_packages(global_args, json);
        },

        Some(Command::Tui) => {
            handlers::handle_cli_default(global_args);
        },

        Some(Command::Search { query, debug,  json }) => {
            handlers::handle_cli_search(global_args, &query, debug, json);
        },

        // Default
        None => {
            handlers::handle_cli_default(global_args);
        }
    }
}
