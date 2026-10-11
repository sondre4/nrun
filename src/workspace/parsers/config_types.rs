use std::collections::HashMap;

use serde::Deserialize;

// For parsing config files
#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub name: Option<String>,
    pub workspaces: Option<Vec<String>>,
    pub scripts: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceConfig {
    pub packages: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct LockfileConfig {
    pub importers: HashMap<String, serde_yaml::Value>,
}
