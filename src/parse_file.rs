use std::{fs, path::Path, thread::panicking};
use serde::Deserialize;
use serde_yaml::Deserializer;

use crate::config::{PackageJsonPartialSchema, PnpmLockfileYamlPartialSchema, PnpmWorkspaceYamlPartialSchema};

pub fn parse_package_json(path: &Path) -> Result<PackageJsonPartialSchema, String> {
    let debug_out_path = path.to_string_lossy();

    println!("trying to parse {debug_out_path}...");

    let json_file = fs::read_to_string(path).unwrap();
    let parsed_json_result: Result<PackageJsonPartialSchema, serde_json::Error> = serde_json::from_str(&json_file);

    match (parsed_json_result) {
        Ok(value) => return Ok(value),
        Err(error) => return Err(format!("Failed to parse '{debug_out_path}': {error}"))
    };
    // if parsedJsonResult.is_err() {
    //     return Err(parsedJsonResult.unwrap_err());
    // }

    // let parsed_json = parsed_json_result.unwrap();
    // return parsedJsonResult;
    // return Ok(parsed_json);
}

pub fn parse_pnpm_workspace_yaml(path: &Path) -> Option<PnpmWorkspaceYamlPartialSchema> {
    let debug_out_path = path.to_string_lossy();

    println!("trying to parse {debug_out_path}...");

    let yaml_file = fs::read_to_string(path).unwrap();
    let parsed_yaml_result: Result<PnpmWorkspaceYamlPartialSchema, serde_yaml::Error> = serde_yaml::from_str(&yaml_file);

    if parsed_yaml_result.is_err() {
        let error = parsed_yaml_result.err();
        println!("Error: {error:?}");
        return None;
    }

    let parsed_yaml = parsed_yaml_result.unwrap();
    return Some(parsed_yaml);
}


pub fn parse_pnpm_lockfile_yaml(path: &Path) -> Vec<PnpmLockfileYamlPartialSchema> {
    let debug_out_path = path.to_string_lossy();
    println!("trying to parse {debug_out_path}...");

    let mut outputPnpmLockfileSchemas: Vec<PnpmLockfileYamlPartialSchema> = Vec::new();

    let yaml_file = fs::read_to_string(path).unwrap();
    let documents = Deserializer::from_str(&yaml_file);

    for document in documents {
        let parsed_yaml_result =  PnpmLockfileYamlPartialSchema::deserialize(document);

        if parsed_yaml_result.is_err() {
            let error = parsed_yaml_result.err();
            println!("Error: {error:?}");
            break;
        }

        let parsed_yaml = parsed_yaml_result.unwrap();
        // return Some(parsed_yaml);
        outputPnpmLockfileSchemas.push(parsed_yaml);
        // if let Some(importers) = parsed_yaml.get("importers") { }
    }


    // let parsed_yaml_result: Result<PnpmLockfileYamlPartialSchema, serde_yaml::Error> = serde_yaml::from_str(&yaml_file);


    // let parsed_yaml = parsed_yaml_result.unwrap();
    return outputPnpmLockfileSchemas;
}
