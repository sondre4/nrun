use std::path::PathBuf;

use crate::workspace::{assembly::search_values::SearchValues, project::paths::get_start_directory};

pub fn initialize_values(_start_directory: Option<PathBuf>, _search_depth: Option<i32>) -> Result<SearchValues, String> {
    let mut start_directory = PathBuf::new();
    let mut search_depth = 7; // default value

    if _start_directory.is_none() {
        let current_directory = get_start_directory();

        match current_directory {
            Ok(directory) => {
                start_directory = directory
            },
            Err(error) => return Err(format!("Failed to find current directory: {error}"))
        };
    }

    if _search_depth.is_some() {
        search_depth = _search_depth.unwrap();
    }

    let initial_values: SearchValues = SearchValues {
        directory: start_directory,
        depth: search_depth
    };

    return Ok(initial_values);
}
