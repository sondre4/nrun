use std::path::PathBuf;

use crate::paths::{self, get_caller_directory};
use crate::package_managers::Project;

pub fn check(path: PathBuf, verbose: bool) -> () {

}

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
}
