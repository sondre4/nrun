// use std::{ffi::c_void, string};

use std::path::PathBuf;

use crate::paths::get_caller_directory;

pub struct Command {
    pub caller_path: PathBuf,
    pub tool_path: PathBuf,
    pub arguments: Vec<String>
}

fn parse_arguments(args: Vec<String>) -> Command {
    let mut command = Command {
        caller_path: PathBuf::new(),
        tool_path: PathBuf::new(),
        arguments: Vec::new()
    };

    // let mut parsed_arguments: Vec<String> = Vec::new();
    let working_directory = get_caller_directory().unwrap(); // unwrap gets the pathbuf from result enum
    command.caller_path = working_directory;

    for arg in args {
        // println!("{}", argument);
        command.arguments.push(arg);
    }

    return command;
}

pub fn handle_command(arguments: Vec<String>) -> () {
    let command: Command = parse_arguments(arguments);

    let length = command.arguments.len();
    let calling_path = command.caller_path.into_string().unwrap();

    println!("Calling directory: {calling_path}");
    println!("arguments: {length}");

    for arg in command.arguments {
        println!("Parsed argument: {arg}");
    }
}
