use std::{ffi::OsString, path::PathBuf};

use crate::{command::script_caller::{ScriptCaller, ScriptCallerError}, workspace::types::{PackageManager, Script}};

#[derive(Debug)]
pub struct ScriptConfig {
    pub package_path: PathBuf,
    pub caller: ScriptCaller,
    pub caller_arguments: Vec<OsString>
}

impl ScriptConfig {
    fn build_caller_arguments(package_manager: &PackageManager, script_name: &String) -> Vec<OsString> {
        let mut args = Vec::new();

        match package_manager {
            PackageManager::Npm => {
                args.push(OsString::from("run"));
                args.push(OsString::from(script_name));
            },

            PackageManager::Pnpm => {
                args.push(OsString::from("run"));
                args.push(OsString::from(script_name));
            },

            PackageManager::Yarn => {
                args.push(OsString::from("run"));
                args.push(OsString::from(script_name));
            },

            PackageManager::Bun => {
                args.push(OsString::from("run"));
                args.push(OsString::from(script_name));
            }
        }

        // if !script_args.is_empty() {
        //     args.push(OsString::from("--"));
        //     args.extend_from_slice(script_args);
        // }

        return args;
    }

    pub fn from_config_data(script: &Script, package_path: &PathBuf, workspace_package_manager: &PackageManager) -> Result<Self, String> {
        // let caller = Self::to_script_caller(workspace_package_manager);
        let script_caller = ScriptCaller::from_package_manager(workspace_package_manager);
        match script_caller {
            Ok(caller) => {
                let mut arguments = Self::build_caller_arguments(workspace_package_manager, &script.name);

                let config: ScriptConfig = ScriptConfig {
                    package_path: package_path.clone(),
                    caller: caller, 
                    caller_arguments: arguments
                };
                
                return Ok(config);
            },
            Err(error) => {
                match error {
                    ScriptCallerError::PackageManagerNotFound { package_manager, searched_names } => {
                        return Err(format!("Package manager not found. Paths tried: {}", searched_names));
                    },
                    ScriptCallerError::PathNotSet => {
                        return Err(format!("Path not set"));
                    },
                    ScriptCallerError::Io(io_error) => {
                        return Err(format!("IO Error: {}", io_error));
                    }
                }
            }
        }
    }
}
