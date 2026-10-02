use crate::workspace::types::{Package, Script};

pub struct ScoredScript  {
    pub script: Script,
    pub script_name_score: f64,
    pub script_command_score: f64,
    pub score: f64
}

pub struct ScoredPackage {
    pub package: Package,
    pub scored_scripts: Vec<ScoredScript>,
    pub package_name_score: f64,
    pub score: f64
}
