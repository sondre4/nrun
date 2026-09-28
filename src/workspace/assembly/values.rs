use std::path::PathBuf;

use crate::workspace::{assembly::search_values::SearchValues, project::paths::get_start_directory, types::Package};

pub fn initialize_values(initial_directory: Option<PathBuf>, initial_search_depth: Option<i32>) -> Result<SearchValues, String> {
    let mut start_directory = PathBuf::new();
    let mut search_depth = 7; // default value

    if initial_directory.is_none() {
        let current_directory = get_start_directory();

        match current_directory {
            Ok(directory) => {
                start_directory = directory
            },
            Err(error) => return Err(format!("Failed to find current directory: {error}"))
        };
    }

    if initial_search_depth.is_some() {
        search_depth = initial_search_depth.unwrap();
    }

    let initial_values: SearchValues = SearchValues {
        directory: start_directory,
        depth: search_depth
    };

    return Ok(initial_values);
}

pub fn get_next_package_id(package: &Package) -> usize {
    return package.id + 1;
}
