use std::path::PathBuf;

use crate::workspace::{
    assembly::values::{get_next_package_id, initialize_values},
    parsers::{package_json::read_package_json, pnpm_workspace::find_pnpm_package_path_patterns},
    project::find::find_monorepo_project,
    resolvers::{path_patterns::resolve_packages, scripts::resolve_scripts},
    types::{Package, PackageManager, Workspace},
};

const ROOT_PACKAGE_ID: usize = 0;

pub fn build_workspace_object(start_directory: Option<PathBuf>, search_depth: u32) -> Result<Workspace, String> {
    // initial values
    let values = initialize_values(start_directory, search_depth)?;
    let directory = values.directory;
    let depth = values.depth;

    // find monorepo
    let monorepo = find_monorepo_project(directory, depth)?;

    let mut workspace: Workspace = Workspace {
        root_path: monorepo.root_path.clone(),
        package_manager: monorepo.package_manager,
        packages: Vec::new(),
    };

    // println!("Found project: '{}' with package manager '{:#?}'", monorepo.root_path.to_string_lossy(), monorepo.package_manager);

    // read root package.json
    // not needed for pnpm? still needed for main scripts
    let root_package_json_result = read_package_json(&monorepo);
    let root_package_json = match root_package_json_result {
        Ok(schema) => schema,
        Err(error) => {
            return Err(format!("Error: {error}"));
        }
    };

    // read scripts in root package.json
    let root_package_json_scripts_result = resolve_scripts(&root_package_json);
    let root_package_json_scripts = match root_package_json_scripts_result {
        Ok(result) => result,
        Err(error) => {
            return Err(format!("Error: {error}"));
        }
    };

    // add root package to workspace vector
    let root_package: Package = Package {
        id: ROOT_PACKAGE_ID,
        name: root_package_json.name.unwrap_or(workspace.root_path.clone().to_string_lossy().to_string()),
        path: workspace.root_path.clone(),
        scripts: root_package_json_scripts,
    };

    workspace.packages.push(root_package);

    // find path patterns of packages

    let mut extra_patterns: Vec<String> = Vec::new();

    match monorepo.package_manager {
        PackageManager::Pnpm => {
            // for pnpm: parse pnpm-workspace.yaml
            let patterns = find_pnpm_package_path_patterns(&monorepo)?;
            extra_patterns = patterns;
        }
        PackageManager::Npm | PackageManager::Bun | PackageManager::Yarn => {
            if root_package_json.workspaces.is_some() {
                // workspaces field in package.json
                extra_patterns = root_package_json.workspaces.unwrap();
            }
        }
    }

    let first_package = workspace.packages.first();
    let mut package_start_id = get_next_package_id(first_package.unwrap());

    // Find packages from path patterns
    for path_pattern in &extra_patterns {
        let found_packages = resolve_packages(&monorepo, &path_pattern, package_start_id)?;

        for found_package in found_packages {
            workspace.packages.push(found_package);
        }

        let last_package_found = workspace.packages.last();

        if last_package_found.is_some() {
            package_start_id = get_next_package_id(last_package_found.unwrap());
        } else {
            package_start_id += workspace.packages.len();
        }
    }

    // println!("Package patterns: {:#?}", extra_patterns);
    // println!("Project found: {:#?}", workspace);

    return Ok(workspace);
}
