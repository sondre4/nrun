use crate::search::scoring_types::{ScoredPackage, ScoredScript};

pub fn only_keep_groups_with_score_above_minimum_score(scored_packages: &mut Vec<ScoredPackage>, minimum_score: f64) -> () {
    scored_packages.retain(|package| {
        return package.score > minimum_score;
    });
}

pub fn only_keep_scripts_with_score_above_minimum_score(scored_scripts: &mut Vec<ScoredScript>, minimum_score: f64) -> () {
    scored_scripts.retain(|script| {
        return script.score > minimum_score;
    });
}
