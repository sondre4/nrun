use nucleo::{Config, Matcher, Utf32Str};

use crate::{search::{matching::search_query_empty, search::search_packages}, workspace::types::Package};

pub mod local_types;
pub mod matching;
pub mod scoring;
pub mod search;



pub struct PackageSearcher {
    matcher: Matcher,
    utf32_buffer: Vec<char>
}

impl PackageSearcher {
    pub fn new() -> Self {
        Self {
            matcher: Matcher::new(Config::DEFAULT),
            utf32_buffer: Vec::new()
        }
    }

    pub fn search(&mut self, packages: &Vec<Package>, query: &str) -> Vec<Package> {
        if search_query_empty(query) {
            let new_packages = packages.clone();
            return new_packages;
        }

        let search_results = search_packages(packages, query);
        return search_results;
    }
}

// TODO: to implement package expansion with search:
// add field in app.tui_state for search_packages_expanded: HashSet<usize>
// this field should be initialized to all packages when typing search query (in operation handler: Operation::SearchOperation(input_request))
// this field should be cleared on search cancel
// when rendering: check if search is active
