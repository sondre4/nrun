use std::path::{Path, PathBuf};

use crate::workspace::{
    config::{BUN_CONFIG, NPM_CONFIG, PNPM_CONFIG, Project, YARN_CONFIG},
    types::PackageManager,
};

fn is_npm_project_root(path: &Path) -> bool {
    let manifest_filepath = path.join(NPM_CONFIG.manifest_name);
    let lockfile_filepath = path.join(NPM_CONFIG.lockfile_name);

    if manifest_filepath.exists() && lockfile_filepath.exists() {
        return true;
    }
    return false;
}

fn is_yarn_project_root(path: &Path) -> bool {
    let manifest_filepath = path.join(YARN_CONFIG.manifest_name);
    let lockfile_filepath = path.join(YARN_CONFIG.lockfile_name);

    if manifest_filepath.exists() && lockfile_filepath.exists() {
        return true;
    }
    return false;
}

fn is_bun_project_root(path: &Path) -> bool {
    let manifest_filepath = path.join(BUN_CONFIG.manifest_name);
    let lockfile_filepath = path.join(BUN_CONFIG.lockfile_name);

    if manifest_filepath.exists() && lockfile_filepath.exists() {
        return true;
    }
    return false;
}

fn is_pnpm_project_root(path: &Path) -> bool {
    let manifest_filepath = path.join(PNPM_CONFIG.manifest_name);
    let lockfile_filepath = path.join(PNPM_CONFIG.lockfile_name);
    let workspace_filepath = path.join(PNPM_CONFIG.workspace_config_name);
    if manifest_filepath.exists() && (lockfile_filepath.exists() || workspace_filepath.exists()) {
        return true;
    }
    return false;
}

fn find_package_manager(path: &Path) -> Option<PackageManager> {
    let npm_found = is_npm_project_root(path);
    let yarn_found = is_yarn_project_root(path);
    let bun_found = is_bun_project_root(path);
    let pnpm_found = is_pnpm_project_root(path);

    let results = vec![npm_found, yarn_found, bun_found, pnpm_found];

    let mut amount_found = 0;

    for result in results {
        if result {
            amount_found += 1;
        }
    }

    if amount_found != 1 {
        return None;
    }

    if npm_found {
        return Some(PackageManager::Npm);
    }

    if yarn_found {
        return Some(PackageManager::Yarn);
    }

    if bun_found {
        return Some(PackageManager::Bun);
    }

    if pnpm_found {
        return Some(PackageManager::Pnpm);
    }

    return None;
}

pub fn build_project_info(current_path: PathBuf, package_manager: PackageManager) -> Project {
    let package_manager_config = match package_manager {
        PackageManager::Npm => NPM_CONFIG,
        PackageManager::Pnpm => PNPM_CONFIG,
        PackageManager::Bun => BUN_CONFIG,
        PackageManager::Yarn => YARN_CONFIG,
    };

    let project: Project = Project {
        package_manager: package_manager,
        root_path: current_path,
        config: package_manager_config,
    };

    return project;
}

fn find_monorepo_project_metadata(start_directory: PathBuf, depth: u32) -> Result<Project, String> {
    let mut directory = start_directory;

    for _i in 0..depth {
        let found_package_manager = find_package_manager(&directory);

        if found_package_manager.is_some() {
            let project = build_project_info(directory, found_package_manager.unwrap());
            return Ok(project);
        }

        let pop_successful = directory.pop();
        if !pop_successful {
            return Err("Did not find a project root directory, reached top level directory".to_string());
        }
    }

    return Err(format!("Did not find a project root directory within depth: {depth}").to_string());
}

pub fn find_monorepo_project(start_directory: PathBuf, search_depth: u32) -> Result<Project, String> {
    let found_monorepo = find_monorepo_project_metadata(start_directory, search_depth)?;
    return Ok(found_monorepo);
}
