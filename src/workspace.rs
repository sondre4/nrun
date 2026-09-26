use std::path::PathBuf;

use crate::workspace::{assembly::build_data::build_workspace_object, types::Workspace};

pub mod parsers;
pub mod project;
pub mod resolvers;

pub mod assembly;

pub mod config;
pub mod types;

pub fn find_workspace(_start_directory: Option<PathBuf>, _search_depth: Option<i32>) -> Result<Workspace, String> {
    let workspace = build_workspace_object(_start_directory, _search_depth)?;
    return Ok(workspace);
}
