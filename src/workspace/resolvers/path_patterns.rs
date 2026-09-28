use std::path::{Path, PathBuf};

use glob::glob;

use crate::workspace::{config::Project, parsers::package_json::{read_package_json, read_package_json_from_path}, resolvers::scripts::resolve_scripts, types::{Package, PackageManager}};

fn path_is_package(monorepo: &Project, path: &Path) -> bool {
    let potential_package_path = path.join(monorepo.config.manifest_name);
    return potential_package_path.exists();
}

fn resolve_pattern(root_path: &Path, path_pattern: &String) -> Result<Vec<PathBuf>, String> {
    let combined_pattern = root_path.join(path_pattern);
    let path_matches_result = glob(combined_pattern.to_str().unwrap());

    let debug_out_path = combined_pattern.to_string_lossy();
    // println!("Trying path pattern: {debug_out_path}");

    let path_matches = match path_matches_result {
        Ok(matches) => matches,
        Err(error) => {
            return Err(format!("Error whilst finding path pattern matches: {error}"));
        }
    };

    let mut found_paths: Vec<PathBuf> = Vec::new();

    for path_match in path_matches {
        match path_match {
            Ok(path) => {
                found_paths.push(path)
            }

            Err(error) => {
                return Err(format!("Error whilst collecting path pattern matches: {error}"));
            }
        }
    }

    return Ok(found_paths);
}

pub fn resolve_packages(monorepo: &Project, path_pattern: &String, start_package_id: usize) -> Result<Vec<Package>, String> {
    let found_paths = resolve_pattern(&monorepo.root_path, path_pattern)?;

    let mut resolved_packages: Vec<Package> = Vec::new();
    let mut current_package_id = start_package_id;

    for found_path in found_paths {

        if path_is_package(monorepo, &found_path) {
            let package_json_path = found_path.join(monorepo.config.manifest_name);
            let package_json = read_package_json_from_path(&package_json_path)?;
            let read_scripts = resolve_scripts(&package_json)?;
            
            let resolved_package: Package = Package {
                id: current_package_id,
                name: package_json.name.unwrap_or(format!("Package {}", current_package_id)),
                path: found_path,
                scripts: read_scripts,
            };

            resolved_packages.push(resolved_package);
            current_package_id += 1;
        }
    }

    return Ok(resolved_packages);
}
