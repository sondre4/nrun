use std::path::PathBuf;

use crate::config::{PackageJsonPartialSchema, Project};
use crate::parse_file::{parse_package_json, parse_pnpm_lockfile_yaml, parse_pnpm_workspace_yaml};
use crate::paths::{self, get_caller_directory};

pub fn check(path: PathBuf, verbose: bool) -> () {}

pub fn default() -> () {
    let called_from: PathBuf = get_caller_directory().unwrap();
    let called_from_path: std::borrow::Cow<'_, str> = called_from.to_string_lossy();
    println!("current path: {called_from_path}");
    let found_project = paths::find_project_base(called_from);

    if found_project.is_none() {
        println!("Did not find project");
        return;
    }

    let project: Project = found_project.unwrap();
    let root_path = project.root_path.to_string_lossy();

    println!("project root: {root_path}");

    let package_json_path = project.root_path.join(project.config.manifest_name);
    let printable_path = package_json_path.to_string_lossy();

    let parsed_package_json = parse_package_json(&package_json_path);

    if parsed_package_json.is_none() {
        println!("Error whilst parsing {printable_path}");
        return;
    }

    let unwrapped = parsed_package_json.unwrap();

    let name = unwrapped.name.unwrap();
    let scripts = unwrapped.scripts;
    let workspaces = unwrapped.workspaces;

    println!("name: {name} ");

    let workspace_yaml_path = project.root_path.join(project.config.workspace_config_name);
    let printable_yaml_path = workspace_yaml_path.to_string_lossy();

    let parsed_workspace_yaml = parse_pnpm_workspace_yaml(&workspace_yaml_path);

    if parsed_workspace_yaml.is_none() {
        println!("Error whilst parsing {printable_yaml_path}");
        return;
    }

    let unwrapped = parsed_workspace_yaml.unwrap();

    println!("parsed workspace yaml: {unwrapped:#?} ");

    let lockfile_yaml_path = project.root_path.join(project.config.lockfile_name);
    let printable_yaml_path = lockfile_yaml_path.to_string_lossy();

    let parsed_lockfiles_yaml = parse_pnpm_lockfile_yaml(&lockfile_yaml_path);

    let vec_length = parsed_lockfiles_yaml.len();
    println!("Got {vec_length} parsed yaml documents after parsing {printable_yaml_path}");

    for parsed_lockfile in parsed_lockfiles_yaml {
        println!("parsed lockfile yaml: {parsed_lockfile:#?} ");
        // for (name, value) in &parsed_lockfile.importers.package_paths {
        //     println!("{name}");
        // }
    }

    // if parsed_lockfile_yaml.is_none() {
    //     return;
    // }

    // let unwrapped = parsed_lockfile_yaml.unwrap();
}
