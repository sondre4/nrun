mod workspace;
mod cli;
mod tui;



use clap::Parser;

fn main() {
    let input = cli::commands::Cli::parse();
    cli::commands::handle_command(input);
}
