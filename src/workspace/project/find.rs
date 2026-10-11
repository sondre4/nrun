use std::path::{Path, PathBuf};

use crate::workspace::{
    config::{BUN_CONFIG, NPM_CONFIG, PNPM_CONFIG, Project, YARN_CONFIG},
    types::PackageManager,
};

fn is_npm_project_root(path: &Path) -> bool {
    let workspace_filepath = path.join(NPM_CONFIG.manifest_name);
    let lockfile_filepath = path.join(NPM_CONFIG.lockfile_name);

    if workspace_filepath.exists() && lockfile_filepath.exists() {
        return true;
    }
    return false;
}

fn is_yarn_project_root(path: &Path) -> bool {
    let workspace_filepath = path.join(YARN_CONFIG.manifest_name);
    let lockfile_filepath = path.join(YARN_CONFIG.lockfile_name);

    if workspace_filepath.exists() && lockfile_filepath.exists() {
        return true;
    }
    return false;
}

fn is_bun_project_root(path: &Path) -> bool {
    let workspace_filepath = path.join(BUN_CONFIG.manifest_name);
    let lockfile_filepath = path.join(BUN_CONFIG.lockfile_name);

    if workspace_filepath.exists() && lockfile_filepath.exists() {
        return true;
    }
    return false;
}

fn is_pnpm_project_root(path: &Path) -> bool {
    let workspace_filepath = path.join(PNPM_CONFIG.workspace_config_name);
    let manifest_filepath = path.join(PNPM_CONFIG.manifest_name);
    let lockfile_filepath = path.join(PNPM_CONFIG.lockfile_name);
    if workspace_filepath.exists() && manifest_filepath.exists() && lockfile_filepath.exists() {
        return true;
    }
    return false;
}

// TODO: add depth option in command
fn find_monorepo_project_metadata(start_directory: PathBuf, depth: u32) -> Result<Project, String> {
    // const DEPTH: i32 = 7;
    let mut directory = start_directory;

    for _i in 0..depth {
        if is_npm_project_root(&directory) {
            let project: Project = Project {
                package_manager: PackageManager::Npm,
                root_path: directory,
                config: NPM_CONFIG,
            };

            return Ok(project);
        }

        if is_yarn_project_root(&directory) {
            let project: Project = Project {
                package_manager: PackageManager::Pnpm,
                root_path: directory,
                config: PNPM_CONFIG,
            };

            return Ok(project);
        }

        if is_bun_project_root(&directory) {
            let project: Project = Project {
                package_manager: PackageManager::Pnpm,
                root_path: directory,
                config: PNPM_CONFIG,
            };

            return Ok(project);
        }

        if is_pnpm_project_root(&directory) {
            let project: Project = Project {
                package_manager: PackageManager::Pnpm,
                root_path: directory,
                config: PNPM_CONFIG,
            };

            return Ok(project);
        }

        let pop_successfully = directory.pop();
        if !pop_successfully {
            return Err("Unable to find project: Reached top level directory".to_string());
        }
    }

    return Err(format!("Unable to find project: Did not find monorepo root within depth: {depth}").to_string());
}

pub fn find_monorepo_project(start_directory: PathBuf, search_depth: u32) -> Result<Project, String> {
    let found_monorepo = find_monorepo_project_metadata(start_directory, search_depth)?;
    return Ok(found_monorepo);
}
