use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::config::{PNPM_CONFIG, Package, PackageJsonPartialSchema, Project, Script, Workspace};
use crate::parse_file::{parse_package_json, parse_pnpm_lockfile_yaml, parse_pnpm_workspace_yaml};
use crate::paths::{self, get_caller_directory};
use crate::workspaces::{get_package_json_schema, get_pnpm_workspace, get_project_info};


fn debug_print_path_from_project(project: &Project) -> () {
    let root_path = project.root_path.to_string_lossy();
    println!("project root: {root_path}");
}


pub fn default() -> () {
    let called_from: PathBuf = get_caller_directory().unwrap();
    let wrapped_project = get_project_info(&called_from);

    if wrapped_project.is_none() {
        println!("Did not find project");
        return;
    }

    let project: Project = wrapped_project.unwrap();
    debug_print_path_from_project(&project);

    let root_package_json_schema_result  = get_package_json_schema(&project);
    let root_package_json_schema = match root_package_json_schema_result {
        Ok(schema) => schema,
        Err(error) => {
            eprintln!("Error: {error}");
            return;
        }
    };

    // if package manager is pnpm:
    let workspace_result = get_pnpm_workspace(&project, &root_package_json_schema);
    let workspace = match workspace_result {
        Ok(workspace) => workspace,
        Err(error) => {
            println!("Error: {error}");
            return;
        }
    };
    
    println!("Workspace: {workspace:#?}");




}

pub fn check(path: PathBuf, verbose: bool) -> () {}
