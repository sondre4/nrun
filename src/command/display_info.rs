use crate::command::script_config::ScriptConfig;
use owo_colors::OwoColorize;
use std::{ffi::OsString, fmt::Display};

fn info<T: Display>(text: T) -> String {
    return text.to_string().bright_cyan().to_string();
}

fn info_strong<T: Display>(text: T) -> String {
    return text.to_string().cyan().to_string();
}

fn text<T: Display>(text: T) -> String {
    return text.to_string().white().to_string();
}

fn text_important<T: Display>(text: T) -> String {
    return text.to_string().bright_white().to_string();
}

fn separator<T: Display>(text: T) -> String {
    return text.to_string().bright_black().to_string();
}

fn print_separator(length: usize) -> () {
    let line = "-".repeat(length);
    let separator = separator(&line);
    println!("{separator}");
}

pub fn display_command_call_info(config: &ScriptConfig) -> () {
    // title
    println!("{}", separator("Nrun"));
    // print_separator();
    // println!();

    // Using ... line
    let using_line_content =
        format!("{} {} {} {}", text("Using"), info(&config.package_manager), text("from"), info_strong(config.caller.program_path.display()));
    println!("{} {}", separator(">"), using_line_content);

    // Running script in ... line
    let running_in_line = format!("{} {}", text("Running script in"), info_strong(config.package_path.display()));
    println!("{} {}", separator(">"), running_in_line);

    // Package script line
    let running_in_line = format!("{} {}", info(&config.package_name), text_important(&config.script_name));
    println!("{} {}\n", separator(">"), running_in_line);

    // Script name (stored as command args)
    let arguments_string = config.caller_arguments.join(OsString::from(" ").as_ref());
    let arguments_display = arguments_string.display();
    let script_line = format!("{} {}", info(&config.package_manager), info(arguments_display));

    // Print line
    print_separator(20);

    // Print script line
    println!("{} {}\n", separator(">"), script_line);
}
