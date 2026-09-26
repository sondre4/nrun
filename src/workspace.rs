use std::path::PathBuf;

use crate::workspace;
use crate::workspace::config::Project;
use crate::workspace::parsers::package_json::read_package_json;
use crate::workspace::parsers::pnpm_workspace::{find_pnpm_package_path_patterns};
use crate::workspace::project::find::find_monorepo_project;
use crate::workspace::project::paths::get_start_directory;
use crate::workspace::resolvers::scripts::get_scripts_from_package_json;
use crate::workspace::types::{Package, PackageManager};
use crate::workspace::{types::Workspace};

pub mod parsers;
pub mod project;
pub mod resolvers;

pub mod config;
pub mod types;

struct SearchValues {
    directory: PathBuf,
    depth: i32
}

fn initialize_values(_start_directory: Option<PathBuf>, _search_depth: Option<i32>) -> Result<SearchValues, String> {
    let mut start_directory = PathBuf::new();
    let mut search_depth = 7; // default value

    if _start_directory.is_none() {
        let current_directory = get_start_directory();

        match current_directory {
            Ok(directory) => {
                start_directory = directory
            },
            Err(error) => return Err(format!("Failed to find current directory: {error}"))
        };
    }

    if _search_depth.is_some() {
        search_depth = _search_depth.unwrap();
    }

    let initial_values: SearchValues = SearchValues {
        directory: start_directory,
        depth: search_depth
    };

    return Ok(initial_values);
}

pub fn find_workspace(_start_directory: Option<PathBuf>, _search_depth: Option<i32>) -> Result<Workspace, String> {

    // initial values
    let values = initialize_values(_start_directory, _search_depth)?;
    let directory = values.directory;
    let depth = values.depth;

    // find monorepo
    let monorepo = find_monorepo_project(directory, depth)?;

    let mut workspace: Workspace = Workspace {
        root_path: monorepo.root_path.clone(),
        package_manager: monorepo.package_manager,
        packages: Vec::new()
    };

    println!("Found project: '{}' with package manager '{:#?}'", monorepo.root_path.to_string_lossy(), monorepo.package_manager);


    // read root package.json
    // not needed for pnpm? still needed for main scripts
    let root_package_json_result  = read_package_json(&monorepo);
    let root_package_json = match root_package_json_result {
        Ok(schema) => schema,
        Err(error) => {
            return Err(format!("Error: {error}"));
        }
    };

    // read scripts in root package.json
    let root_package_json_scripts_result = get_scripts_from_package_json(&root_package_json);
    let root_package_json_scripts = match root_package_json_scripts_result {
        Ok(result) => result,
        Err(error) => {
            return Err(format!("Error: {error}"));
        }
    };

    // add root package to workspace vector
    let root_package: Package = Package {
        name: root_package_json.name.unwrap_or("Project root".to_string()),
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
            // check workspaces field in root package.json
        }
    }

    println!("Package patterns: {:#?}", extra_patterns);
    println!("Project found: {:#?}", workspace);

    // TODO: check the 

    return Err("Not implemented".to_string());
}
