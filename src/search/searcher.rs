use nucleo::{Config, Matcher, pattern::{CaseMatching, Normalization, Pattern}};

use crate::{search::{
    filtering::{
        only_keep_groups_with_score_above_minimum_score,
        only_keep_scripts_with_score_above_minimum_score}, matching::search_query_empty, scoring::{
        calculate_minimum_needed_score,
        get_scores_from_scored_packages,
        get_scores_from_scored_scripts,
        score_package,
        should_filter_package_scripts,
        sort_packages_by_score,
        sort_scripts_by_score
    }, scoring_types::{
        ScoredPackage,
        ScoredScript
        }
    }, workspace::types::{Package, Script}
};

pub struct PackageSearcher {
    matcher: Matcher,
    utf32_buffer: Vec<char>,
    display_debug_info: bool
}

impl PackageSearcher {
    pub fn new() -> Self {
        Self {
            matcher: Matcher::new(Config::DEFAULT),
            utf32_buffer: Vec::new(),
            display_debug_info: false
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

        if self.display_debug_info {
            println!("\nMinimum package score needed: {minimum_needed_score}");
        }

        only_keep_groups_with_score_above_minimum_score(sorted_scored_packages, minimum_needed_score);
    }

    fn filter_scored_scripts(&mut self, sorted_scored_scripts: &mut Vec<ScoredScript>) {
        let scored_script_scores = get_scores_from_scored_scripts(sorted_scored_scripts);
        let minimum_needed_script_score = calculate_minimum_needed_score(&scored_script_scores);

        if self.display_debug_info {
            println!("\nMinimum script score needed: {minimum_needed_script_score}");
        }

        only_keep_scripts_with_score_above_minimum_score(sorted_scored_scripts, minimum_needed_script_score);
    }

    fn process_scored_package_scripts(&mut self, scored_package: &mut ScoredPackage) -> () {
        if self.display_debug_info {
            println!("-------------\nscripts for package: {}", scored_package.package.name);
        }

        // first: sort the scored package's scripts
        sort_scripts_by_score(&mut scored_package.scored_scripts);

        if self.display_debug_info {
            println!("After sorting");
            self.debug_display_script_scores(&scored_package.scored_scripts);
        }

        // check if scored package's scripts need to be filtered
        let filter_package_scripts = should_filter_package_scripts(&scored_package);

        if self.display_debug_info {
            println!("\n------------\nfilter package scripts: {}", filter_package_scripts);
        }

        if filter_package_scripts {
            self.filter_scored_scripts(&mut scored_package.scored_scripts);
        }

        if self.display_debug_info {
            println!("\n------------\nFinished filtering scripts:");
            self.debug_display_script_scores(&scored_package.scored_scripts);
        }
    }

    pub fn search(&mut self, packages: &Vec<Package>, query: &str) -> Vec<Package> {
        if search_query_empty(query) {
            let new_packages = packages.clone();
            return new_packages;
        }

        let mut scored_packages = self.score_packages(packages, query);

        if self.display_debug_info {
            println!("After scoring packages:");
            self.debug_display_package_scores(&scored_packages);
        }

        sort_packages_by_score(&mut scored_packages);

        if self.display_debug_info {
            println!("\n------------------\nAfter sorting by score:");
            self.debug_display_package_scores(&scored_packages);
        }

        self.filter_scored_packages(&mut scored_packages);

        if self.display_debug_info {
            println!("\n------------------\nAfter filtering:");
            self.debug_display_package_scores(&scored_packages);
        }

        // define vector to be returned
        let mut search_results: Vec<Package> = Vec::new();

        // filter scripts in each scored package
        for mut package in scored_packages {
            self.process_scored_package_scripts(&mut package);

            if self.display_debug_info {
                println!("\n\n\n---------------------------------\nSCRIPTS | Package: {}\n----------------------------------\nScripts before adding to search results:",
                    package.package.name
                );
                self.debug_display_script_scores(&package.scored_scripts);
            }

            // remember to store the package's filtered scripts
            let mut package_scripts: Vec<Script> = Vec::new();
            
            for scored_script in package.scored_scripts {
                package_scripts.push(scored_script.script);
            }

            let mut package: Package = package.package;
            package.scripts = package_scripts;

            search_results.push(package);
        }

        // let search_results = search_packages(packages, query);
        return search_results;
    }

    pub fn enable_debug(&mut self) -> () {
        self.display_debug_info = true;
    }

    fn debug_display_package_scores(&self, scored_packages: &Vec<ScoredPackage>) -> () {
        for package in scored_packages {
            println!("total_score: {:<6.2} | name_score: {:<6.2} | package: {}", &package.score, &package.package_name_score, &package.package.name);
        }
    }

    fn debug_display_script_scores(&self, scored_scripts: &Vec<ScoredScript>) -> () {
        for script in scored_scripts {
            println!("total_score: {:<6.2} | name_score: {:<6.2} command_score: {:<6.2} | script_name: {}", &script.score, &script.script_name_score, &script.script_command_score, &script.script.name);
        }
    }
}
