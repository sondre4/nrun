use std::path::PathBuf;

use crate::config::{PackageJsonPartialSchema, Project};
use crate::parse_file::parse_package_json;
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

    let parsedPackageJson = parse_package_json(&package_json_path);

    if parsedPackageJson.is_none() {
        println!("Error whilst parsing {printable_path}");
        return;
    }

    let unwrapped = parsedPackageJson.unwrap();

    let name = unwrapped.name.unwrap();
    let scripts = unwrapped.scripts;
    let workspaces = unwrapped.workspaces;

    println!("name: {name} ");
}
