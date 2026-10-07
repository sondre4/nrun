use std::{process::{Command, Stdio}};

use crate::command::{display_info::display_command_call_info, script_config::ScriptConfig};

pub struct CommandCaller;

impl CommandCaller {
    pub fn call(config: ScriptConfig) -> Result<(), String> {
        display_command_call_info(&config);

        let process_result = Command::new(config.caller.program_path)
            .args(config.caller_arguments)
            .current_dir(config.package_path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match process_result {
            Ok(_result) => {
                return Ok(());
            },

            Err(error) => return Err(format!("Error whilst calling script: {error}"))
        }
    }
}
