use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

use crate::cli::commands::Command::Tui;
use crate::cli::handlers;

#[derive(Parser)]
#[command(
    name = "nrun",
    about = "Calling 'nrun' without a command/subcommand launches the TUI",
    long_about = "about2",
    after_help = "about3",
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    // default command arguments
    /// Optional: specify a path to a workspace root
    #[arg(short, long, global = true)]
    path: Option<PathBuf>,

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

        /// Enable scoring/filtering debug output
        #[arg(long, default_value_t = false)]
        debug: bool
    },

    /// List all packages and their scripts in a workspace
    #[command(visible_alias = "l", visible_alias = "ls")]
    List
}

pub struct GlobalArguments {
    pub path: Option<PathBuf>,
    pub depth: u32
}

impl GlobalArguments {
    pub fn from(input: &Cli) -> GlobalArguments {
        return GlobalArguments {
            path: input.path.clone(),
            depth: input.depth.clone()
        };
    }
}

pub fn handle_command(input: Cli) -> () {
    let global_args = GlobalArguments::from(&input);

    match input.command {
        Some(Command::List) => {
            handlers::handle_list_packages(global_args);
        },

        Some(Command::Tui) => {
            handlers::handle_launching_tui(global_args);
        },

        Some(Command::Search { query, debug }) => {
            handlers::handle_cli_search(global_args, &query, debug);
        },

        None => {
            handlers::handle_launching_tui(global_args);
        }
    }
}
