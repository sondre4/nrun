use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

use crate::commands;

#[derive(Parser)]
pub struct CliInput {
    #[command(subcommand)]
    pub command: Option<Command>
}


#[derive(Subcommand)]
pub enum Command {
    Check {
        path: PathBuf,

        #[arg(short, long)]
        verbose: bool
    }
}

// #[derive(clap::Args)]
// pub struct CheckArgs {
//     path: String,

//     #[arg(short, long)]
//     pub verbose: bool,
// }


pub fn select_command(input: CliInput) -> () {
    match input.command {
        Some(Command::Check { path, verbose }) => {
            commands::check(path, verbose);
        }

        None => {
            commands::find_package_json();
        }
    }
}
