// use std::path::PathBuf;

use std::path::{Path, PathBuf};

use crate::package_managers::{NPM_CONFIG, PACKAGE_MANAGERS, PNPM_CONFIG, PackageManager, Project};

pub fn get_caller_directory() -> Result<std::path::PathBuf, std::io::Error> {
    let path = std::env::current_dir();
    return path;
}

fn is_npm_project_root(path: &Path) -> bool {
    let workspace_filepath = path.join(NPM_CONFIG.manifest_name);
    let lockfile_filepath = path.join(NPM_CONFIG.lockfile_name);
    if workspace_filepath.exists() && lockfile_filepath.exists() {
        return true
    }
    return false;
}

fn is_pnpm_project_root(path: &Path) -> bool {
    let workspace_filepath = path.join(PNPM_CONFIG.workspace_config_name);
    let manifest_filepath = path.join(PNPM_CONFIG.manifest_name);
    let lockfile_filepath = path.join(PNPM_CONFIG.lockfile_name);
    if workspace_filepath.exists() && manifest_filepath.exists() && lockfile_filepath.exists() {
        return true
    }
    return false;
}

pub fn find_project_base(start_directory: PathBuf) -> Option<Project> {
    const DEPTH: i32 = 4;
    let mut directory = start_directory;

    for i in 0..DEPTH {
        if is_npm_project_root(&directory) {
            let project: Project = Project {
                package_manager: PackageManager::Npm,
                root_path: directory,
            };

            return Some(project);
        }

        if is_pnpm_project_root(&directory) {
            let project: Project = Project {
                package_manager: PackageManager::Pnpm,
                root_path: directory,
            };

            return Some(project);
        }

        directory.pop();
    }

    return None;
}
