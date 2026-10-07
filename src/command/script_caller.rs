use std::{io, path::PathBuf};

use crate::workspace::types::PackageManager;

#[derive(Debug)]
pub struct ScriptCaller {
    pub packager_manager: PackageManager,
    pub program_path: PathBuf
}

#[derive(Debug)]
pub enum ScriptCallerError {
    PackageManagerNotFound {
        package_manager: PackageManager,
        searched_names: String,
    },
    PathNotSet,
    Io(io::Error)
}

#[derive(Debug)]
pub struct ScriptConfig {
    pub package_path: PathBuf,
    pub script_caller_path: ScriptCaller,
    pub caller_arguments: Vec<String>
}

impl ScriptCaller {
    pub fn from_package_manager(package_manager: &PackageManager) -> Result<Self, ScriptCallerError> {
        let path = Self::resolve_caller_path(package_manager)?;

        return Ok(ScriptCaller {
            packager_manager: package_manager.clone(),
            program_path: path,
        });
    }

    // names to should look for on PATH.
    fn get_caller_names(packager_manager: &PackageManager) -> &'static [&'static str] {
        #[cfg(windows)]
        {
            use crate::workspace::types::PackageManager;

            match packager_manager {
                PackageManager::Npm => &["npm.cmd", "npm.exe"],
                PackageManager::Pnpm => &["pnpm.cmd", "pnpm.exe"],
                PackageManager::Yarn => &["yarn.cmd", "yarn.exe"],
                PackageManager::Bun => &["bun.exe"],
            }
        }

        #[cfg(not(windows))]
        {
            match packager_manager {
                PackageManager::Npm => &["npm"],
                PackageManager::Pnpm => &["pnpm"],
                PackageManager::Yarn => &["yarn"],
                PackageManager::Bun => &["bun"],
            }
        }
    }

    fn find_on_path(names: &[&str]) -> Result<PathBuf, ScriptCallerError> {
        let path_var = std::env::var_os("PATH");

        if path_var.is_none() {
            return Err(ScriptCallerError::PathNotSet);
        }

        for directory in std::env::split_paths(&path_var.unwrap()) {
            for name in names {
                let possible_file = directory.join(name);

                if possible_file.is_file() {
                    return Ok(possible_file);
                }
            }
        }

        return Err(ScriptCallerError::Io(io::Error::new(io::ErrorKind::NotFound, format!("could not find any of {:?} on PATH", names))));
    }

    fn resolve_caller_path(packager_manager: &PackageManager) -> Result<PathBuf, ScriptCallerError> {
        let names = Self::get_caller_names(packager_manager);
        let found_path = Self::find_on_path(names)?;
        return Ok(found_path);
    }
}
