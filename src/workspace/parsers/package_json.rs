use std::{fs, path::Path};

use crate::workspace::{
    config::Project,
    parsers::{config_types::PackageJson, file_exists::file_exists},
};

fn parse_package_json(path: &Path) -> Result<PackageJson, String> {
    file_exists(path)?;

    let debug_out_path = path.to_string_lossy();

    // println!("trying to parse {debug_out_path}...");

    let json_file = fs::read_to_string(path).unwrap();
    let parsed_json_result: Result<PackageJson, serde_json::Error> = serde_json::from_str(&json_file);

    match (parsed_json_result) {
        Ok(value) => return Ok(value),
        Err(error) => return Err(format!("Failed to parse '{debug_out_path}': {error}")),
    };
}

pub fn read_package_json_from_path(path: &Path) -> Result<PackageJson, String> {
    let parsed_package_json = parse_package_json(path)?;
    return Ok(parsed_package_json);
}

pub fn read_package_json(project: &Project) -> Result<PackageJson, String> {
    let package_json_path = project.root_path.join(project.config.manifest_name);
    let package_json = read_package_json_from_path(&package_json_path)?;
    return Ok(package_json);
}
