use nucleo::{Config, Matcher, pattern::{CaseMatching, Normalization, Pattern}};

use crate::{search::{
    filtering::{
        only_keep_groups_with_score_above_minimum_score,
        only_keep_scripts_with_score_above_minimum_score},
    matching::search_query_empty,
    scoring::{
        calculate_minimum_needed_score,
        get_scores_from_scored_packages,
        get_scores_from_scored_scripts,
        package_name_scored_most,
        score_package,
        sort_packages_by_score,
        sort_scripts_by_score
    },
    scoring_types::{
        ScoredPackage,
        ScoredScript
        }
    },
    workspace::types::Package
};

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

    fn score_packages(&mut self, packages: &Vec<Package>, query: &str) -> Vec<ScoredPackage> {
        let mut scored_packages: Vec<ScoredPackage> = Vec::new();

        // initialize pattern and utf32_buffer
        let query_pattern: Pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);

        for package in packages {
            let scored_package = score_package(
                &query_pattern,
                &mut self.utf32_buffer,
                &mut self.matcher,
                package
            );

            scored_packages.push(scored_package);
        }

        return scored_packages;
    }

    fn filter_scored_packages(&mut self, sorted_scored_packages: &mut Vec<ScoredPackage>) -> () {
        let scored_package_scores = get_scores_from_scored_packages(&sorted_scored_packages);
        let minimum_needed_score = calculate_minimum_needed_score(&scored_package_scores);
        only_keep_groups_with_score_above_minimum_score(sorted_scored_packages, minimum_needed_score);
    }

    fn filter_scored_scripts(&mut self, sorted_scored_scripts: &mut Vec<ScoredScript>) {
        let scored_script_scores = get_scores_from_scored_scripts(sorted_scored_scripts);
        let minimum_needed_script_score = calculate_minimum_needed_score(&scored_script_scores);
        only_keep_scripts_with_score_above_minimum_score(sorted_scored_scripts, minimum_needed_script_score);
    }

    fn process_scored_package_scripts(&mut self, scored_package: &mut ScoredPackage) -> () {

        // first: sort the scored package's scripts
        sort_scripts_by_score(&mut scored_package.scored_scripts);

        // check if scored package's scripts need to be filtered
        let keep_all_package_scripts = package_name_scored_most(&scored_package);

        if !keep_all_package_scripts {
            self.filter_scored_scripts(&mut scored_package.scored_scripts);
        }
    }

    pub fn search(&mut self, packages: &Vec<Package>, query: &str) -> Vec<Package> {
        if search_query_empty(query) {
            let new_packages = packages.clone();
            return new_packages;
        }

        let mut scored_packages = self.score_packages(packages, query);

        sort_packages_by_score(&mut scored_packages);

        self.filter_scored_packages(&mut scored_packages);

        // define vector to be returned
        let mut search_results: Vec<Package> = Vec::new();

        // filter scripts in each scored package
        for mut package in scored_packages {
            self.process_scored_package_scripts(&mut package);
            search_results.push(package.package);
        }

        // let search_results = search_packages(packages, query);
        return search_results;
    }
}
