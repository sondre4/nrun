use std::path::PathBuf;

use crate::workspace::{assembly::search_values::SearchValues, project::paths::get_start_directory, types::Package};

pub fn initialize_values(initial_directory: Option<PathBuf>, initial_search_depth: u32) -> Result<SearchValues, String> {
    let start_directory = if initial_directory.is_some() {
        initial_directory.unwrap()
    } else {
        let current_directory = get_start_directory()?;
        current_directory
    };

    let initial_values: SearchValues = SearchValues {
        directory: start_directory,
        depth: initial_search_depth,
    };

    return Ok(initial_values);
}

pub fn get_next_package_id(package: &Package) -> usize {
    return package.id + 1;
}
