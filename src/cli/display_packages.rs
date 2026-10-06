use owo_colors::OwoColorize;

use crate::workspace::types::{Package, Workspace};

const HEADER_START: usize = 2;
// const VERTICAL_BAR_OFFSET: usize = 2;
const ID_WIDTH: usize = 5;
const SCRIPT_NAME_WIDTH: usize = 25;
const SCRIPT_COMMAND_WIDTH: usize = 60;

fn display_header() {
    println!("{}{:<ID_WIDTH$} {:<NAME_WIDTH$} {}{}", " ".repeat(HEADER_START), "(id)".dimmed(), "Script", "│ ".dimmed(), "Command", ID_WIDTH = (ID_WIDTH + 4), NAME_WIDTH = SCRIPT_NAME_WIDTH);
    println!("{}", ("-".repeat(ID_WIDTH + SCRIPT_NAME_WIDTH + SCRIPT_COMMAND_WIDTH + 2)).dimmed());
}

fn display_package(packages: &Package) {
    // packages
    println!("{} {} {}", "v".cyan().bold(), format!("({})",  packages.id).dimmed(), packages.name.bold().cyan());

    // scripts
    for (index, script) in packages.scripts.iter().enumerate() {
        let branch = if index == packages.scripts.len() - 1 {
            "└─"
        } else {
            "├─"
        };

        let script_name = truncate(&script.name, SCRIPT_NAME_WIDTH);
        let script_command = truncate(&script.command, SCRIPT_COMMAND_WIDTH);

        println!(
            "   {} {:<ID_WIDTH$} {:<NAME_WIDTH$} {} {}",
            branch.dimmed(),
            format!("({})", script.id).dimmed(),
            script_name.bold(),
            "│".dimmed(),
            script_command,
            ID_WIDTH = ID_WIDTH,
            NAME_WIDTH = SCRIPT_NAME_WIDTH
        );
    }
}

fn truncate(value: &str, max_width: usize) -> String {
    if value.chars().count() <= max_width {
        return value.to_string();
    }

    let mut result: String = value.chars().take(max_width.saturating_sub(3)).collect();

    result.push_str("...");
    return result;
}

pub fn display_packages(packages: &Vec<Package>) {
    if packages.is_empty() {
        println!("No results found.");
        return;
    }

    display_header();

    for package in packages {
        display_package(package);
        println!();
    }
}
