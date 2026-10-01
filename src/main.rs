mod cli;
mod search;
mod tui;
mod workspace;

use clap::Parser;

fn main() {
    let input = cli::commands::Cli::parse();
    cli::commands::handle_command(input);
}
