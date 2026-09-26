use std::{fs, path::Path};

use crate::workspace::{config::Project, parsers::file_exists::file_exists, types::WorkspaceConfig};

fn parse_pnpm_workspace_from_path(path: &Path) -> Result<WorkspaceConfig, String> {
    file_exists(path)?;

    let debug_out_path = path.to_string_lossy();

    println!("trying to parse {debug_out_path}...");

    let yaml_file = fs::read_to_string(path).unwrap();
    let parsed_yaml_result: Result<WorkspaceConfig, serde_yaml::Error> = serde_yaml::from_str(&yaml_file);

    match parsed_yaml_result {
        Ok(config) => return Ok(config),
        Err(error) => return Err(format!("Error whilst parsing {debug_out_path}: {error}"))
    }
}

pub fn read_pnpm_workspace_config(project: &Project) -> Result<WorkspaceConfig, String> {
    let path = project.root_path.join(project.config.workspace_config_name);
    let read_config = parse_pnpm_workspace_from_path(&path)?;
    return Ok(read_config);
}
