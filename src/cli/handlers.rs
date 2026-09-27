use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::tui::launch_tui;
use crate::workspace::find_workspace;

// fn debug_print_path_from_project(project: &Project) -> () {
//     let root_path = project.root_path.to_string_lossy();
//     println!("project root: {root_path}");
// }

pub fn default(path: Option<PathBuf>, depth: Option<i32>) -> () {
    let workspace_result = find_workspace(path, depth);
    let workspace = match workspace_result {
        Ok(result) => result,
        Err(error) => {
            println!("Unable to find workspace: {error}");
            return;
        }
    };

    launch_tui(workspace);
}

pub fn check(path: PathBuf, verbose: bool) -> () {
    println!("Not implemented");
}
