use clap::Parser;
use clap::Subcommand;

use crate::commands;

#[derive(Parser)]
pub struct CliInput {
    #[command(subcommand)]
    pub command: Command
}


#[derive(Subcommand)]
pub enum Command {
    Check {
        path: String,

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

pub fn select_command(cliInput: CliInput) -> () {
    match cliInput.command {
        Command::Check { path, verbose } => {
            commands::check(path, verbose);
        }
    }
}
