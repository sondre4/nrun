use nucleo::{Matcher, pattern::Pattern};

use crate::{search::{local_types::{ScoredPackage, ScoredScript}, matching::match_find_score}, workspace::types::{Package, Script}};

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
        score: score_max(name_score, description_score)
    };

    return scored_script;
}

pub fn score_package(pattern: &Pattern, utf32_buffer: &mut Vec<char>, matcher: &mut Matcher, package: &Package) -> ScoredPackage {
    let mut scored_scripts: Vec<ScoredScript> = Vec::new();

    for script in &package.scripts {
        let scored_script = score_script(pattern, utf32_buffer, matcher, script);
        scored_scripts.push(scored_script);
    }

    let name_score = match_find_score(pattern, utf32_buffer, matcher, &package.name);

    let scored_package: ScoredPackage = ScoredPackage {
        package: package.clone(),
        scored_scripts: scored_scripts,
        package_name_score: name_score,
        score: name_score
    };

    return scored_package;
}
