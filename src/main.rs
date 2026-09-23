mod commands;
mod paths;
mod cli;

use clap::Parser;

fn main() {
    // let arguments: Vec<String> = std::env::args().collect();
    // let arguments = cli::Arguments::parse();
    // commands::handle_command(arguments);

    let input = cli::CliInput::parse();
    cli::select_command(input);
}
