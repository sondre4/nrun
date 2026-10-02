use nucleo::{Matcher, pattern::Pattern};

use crate::{search::{matching::match_find_score, scoring_types::{MINIMUM_RELATIVE_SCORE_FACTOR, PACKAGE_NAME_SCORE_MULTIPLIER, SIGNIFICANT_SCORE_GAP, ScoredPackage, ScoredScript}}, workspace::types::{Package, Script}};

pub fn get_scores_from_scored_scripts(scored_scripts: &Vec<ScoredScript>) -> Vec<f64> {
    let mut scores: Vec<f64> = Vec::new();

    for script in scored_scripts {
        scores.push(script.score);
    }

    return scores;
}

pub fn get_scores_from_scored_packages(scored_packages: &Vec<ScoredPackage>) -> Vec<f64> {
    let mut scores: Vec<f64> = Vec::new();

    for package in scored_packages {
        scores.push(package.score);
    }

    return scores;
}

// packages with a score lower than this will not be included
pub fn calculate_minimum_needed_score(scores: &Vec<f64>) -> f64 {
    if scores.is_empty() {
        return 0.0;
    }

    let scores_length = scores.len();

    if scores_length == 1 {
        return scores[0];
    }

    // scores sorted in descending order
    let best_score = scores[0];

    let mut largest_gap = 0.0;
    let mut largest_gap_index = None;

    for i in 1..scores_length {
        let previous = scores[i - 1];
        let current = scores[i];

        if previous <= 0.0 {
            continue;
        }

        let gap = (previous - current) / previous;

        if gap > largest_gap {
            largest_gap = gap;
            largest_gap_index = Some(i);
        }
    }

    if largest_gap >= SIGNIFICANT_SCORE_GAP {
        if let Some(index) = largest_gap_index {
            return scores[index - 1];
        }
    }

    return best_score * MINIMUM_RELATIVE_SCORE_FACTOR;
}


// descending: largest first
pub fn sort_scripts_by_score(scripts: &mut Vec<ScoredScript>) -> () {
    scripts.sort_by(|a, b| {
        b.score.total_cmp(&a.score)
    });
}

// descending: largest first
pub fn sort_packages_by_score(packages: &mut Vec<ScoredPackage>) -> () {
    packages.sort_by(|a, b| {
        b.score.total_cmp(&a.score)
    });
}

pub fn package_name_scored_most(scored_package: &ScoredPackage) -> bool {
    let result = scored_package.package_name_score > scored_package.score;
    return result;
}

fn score_max(score1: f64, score2: f64) -> f64 {
    if score1 > score2 {
        return score1;
    }

    return score2;
}

pub fn score_script(pattern: &Pattern, utf32_buffer: &mut Vec<char>, matcher: &mut Matcher, script: &Script) -> ScoredScript {
    let name_score = match_find_score(pattern, utf32_buffer, matcher, &script.name);
    let description_score = match_find_score(pattern, utf32_buffer, matcher, &script.command);

    let scored_script: ScoredScript = ScoredScript {
        script: script.clone(),
        script_name_score: name_score,
        script_command_score: description_score,
        score: score_max(name_score, description_score) // multiply with weights here?
    };

    return scored_script;
}

pub fn score_package(pattern: &Pattern, utf32_buffer: &mut Vec<char>, matcher: &mut Matcher, package: &Package) -> ScoredPackage {
    let mut scored_scripts: Vec<ScoredScript> = Vec::new();
    let mut best_item_score: f64 = 0.0f64;

    for script in &package.scripts {
        let scored_script = score_script(pattern, utf32_buffer, matcher, script);
        best_item_score = score_max(best_item_score, scored_script.score);
        scored_scripts.push(scored_script);
    }

    let name_score = match_find_score(pattern, utf32_buffer, matcher, &package.name);

    let scored_package: ScoredPackage = ScoredPackage {
        package: package.clone(),
        scored_scripts: scored_scripts,
        package_name_score: name_score * PACKAGE_NAME_SCORE_MULTIPLIER,
        score: score_max(name_score, best_item_score) // multiply with weights here?
    };

    return scored_package;
}
