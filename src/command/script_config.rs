use std::{ffi::OsString, path::PathBuf};

use crate::{
    command::script_caller::{ScriptCaller, ScriptCallerError},
    workspace::types::{Package, PackageManager, Script},
};

#[derive(Debug)]
pub struct ScriptConfig {
    pub package_manager: String,
    pub package_name: String,
    pub script_name: String,
    pub package_path: PathBuf,
    pub caller: ScriptCaller,
    pub caller_arguments: Vec<OsString>,
}

impl ScriptConfig {
    fn build_caller_arguments(package_manager: &PackageManager, script_name: &String) -> Vec<OsString> {
        let mut args = Vec::new();

        match package_manager {
            PackageManager::Npm => {
                args.push(OsString::from("run"));
                args.push(OsString::from(script_name));
            }

            PackageManager::Pnpm => {
                args.push(OsString::from("run"));
                args.push(OsString::from(script_name));
            }

            PackageManager::Yarn => {
                args.push(OsString::from("run"));
                args.push(OsString::from(script_name));
            }

            PackageManager::Bun => {
                args.push(OsString::from("run"));
                args.push(OsString::from(script_name));
            }
        }

        return args;
    }

    pub fn from_config_data(script: &Script, package: &Package, workspace_package_manager: &PackageManager) -> Result<Self, String> {
        let script_caller = ScriptCaller::from_package_manager(workspace_package_manager);
        match script_caller {
            Ok(caller) => {
                let arguments = Self::build_caller_arguments(workspace_package_manager, &script.name);

                let config: ScriptConfig = ScriptConfig {
                    package_manager: workspace_package_manager.to_string(),
                    package_name: package.name.clone(),
                    script_name: script.name.clone(),
                    package_path: package.path.clone(),
                    caller: caller,
                    caller_arguments: arguments,
                };

                return Ok(config);
            }
            Err(error) => match error {
                // ScriptCallerError::PackageManagerNotFound {
                //     package_manager: _,
                //     searched_names,
                // } => {
                //     return Err(format!("Package manager not found. Paths tried: {}", searched_names));
                // }
                ScriptCallerError::PathNotSet => {
                    return Err("Path not set".to_string());
                }
                ScriptCallerError::Io(io_error) => {
                    return Err(format!("{} (hint: does your project's package manager exist in your environment?)", io_error));
                }
            },
        }
    }
}
