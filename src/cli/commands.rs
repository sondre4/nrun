use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

use crate::cli::handlers;


#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    // default command arguments
    #[arg(short, long)]
    path: Option<PathBuf>,

    #[arg(short, long)]
    depth: Option<i32>
}

#[derive(Subcommand)]
pub enum Command {
    Check {
        path: PathBuf,

        #[arg(short, long)]
        verbose: bool
    }
}

pub fn handle_command(input: Cli) -> () {
    match input.command {
        Some(Command::Check { path, verbose }) => {
            handlers::check(path, verbose);
        }

        None => {
            handlers::default(input.path, input.depth);
        }
    }
}
