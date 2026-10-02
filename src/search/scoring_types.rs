use crate::workspace::types::{Package, Script};

pub const SIGNIFICANT_SCORE_GAP: f64 = 0.25;
pub const MINIMUM_RELATIVE_SCORE_FACTOR: f64 = 0.5;

pub struct ScoredScript  {
    pub script: Script,
    pub script_name_score: f64,
    pub script_command_score: f64,
    pub score: f64
}

pub struct ScoredPackage {
    pub package: Package,
    pub scored_scripts: Vec<ScoredScript>,

    // did package match because of its name?
    pub package_name_score: f64,

    // max score of either one of package's scripts or its name
    pub score: f64
}
