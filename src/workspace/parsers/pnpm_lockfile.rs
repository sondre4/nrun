use std::{fs, path::Path};

use serde::Deserialize;
use serde_yaml::Deserializer;

use crate::workspace::{parsers::file_exists::file_exists, types::LockfileConfig};

pub fn parse_pnpm_lockfile_yaml(path: &Path) -> Result<Vec<LockfileConfig>, String> {
    file_exists(path)?;

    let debug_out_path = path.to_string_lossy();
    // println!("trying to parse {debug_out_path}...");

    let mut output_pnpm_lockfile_schemas: Vec<LockfileConfig> = Vec::new();

    let yaml_file = fs::read_to_string(path).unwrap();
    let documents = Deserializer::from_str(&yaml_file);

    for document in documents {
        let parsed_yaml_result =  LockfileConfig::deserialize(document);

        match parsed_yaml_result {
            Ok(parsed_yaml) => {
                output_pnpm_lockfile_schemas.push(parsed_yaml);
            },
            Err(error ) => return Err(format!("Error whilst parsing {debug_out_path}: {error}"))
        }
    }

    return Ok(output_pnpm_lockfile_schemas);
}
