use std::{ffi::OsString, process::{Command, Stdio}};
use owo_colors::OwoColorize;

use crate::command::script_config::ScriptConfig;

pub struct CommandCaller;

impl CommandCaller {
    fn display_command(config: &ScriptConfig) -> () {
        // Running commands:
        // > cd package_path
        // > caller.path <args>

        //
        eprintln!("nrun: running:");

        // cd
        let directory_display = config.package_path.display();
        eprintln!("{} {}", ">".bright_blue(), format!("cd {directory_display}").bright_white());

        // run
        let program_path_display = config.caller.program_path.display();
        let arguments_string = config.caller_arguments.join(OsString::from(" ").as_ref());
        let arguments_lossy_string = arguments_string.to_string_lossy();
        // let arguments_display = arguments_string.display();

        eprintln!("{} {}\r", ">".bright_blue(), format!("{program_path_display} {arguments_lossy_string}", ).bright_white());
    }

    pub fn call(config: ScriptConfig) -> Result<(), String> {
        Self::display_command(&config);

        let process_result = Command::new(config.caller.program_path)
            .args(config.caller_arguments)
            .current_dir(config.package_path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            // .spawn();
            .status();

        match process_result {
            Ok(_result) => {
                eprintln!("\r\n");
                return Ok(());
            },

            Err(error) => return Err(format!("Error whilst calling script: {error}"))
        }
    }
}
