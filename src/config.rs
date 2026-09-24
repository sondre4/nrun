use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

pub struct PackageManagerConfig {
    pub manifest_name: &'static str,
    pub lockfile_name: &'static str,
    pub workspace_config_name: &'static str,
    pub workspace_config_field: &'static str
}

pub const NPM_CONFIG: PackageManagerConfig = PackageManagerConfig {
    manifest_name: "package.json",
    lockfile_name: "package-lock.json",
    workspace_config_name: "",
    workspace_config_field: "workspaces",
};

pub const PNPM_CONFIG: PackageManagerConfig = PackageManagerConfig {
    manifest_name: "package.json",
    lockfile_name: "pnpm-lock.yaml",
    workspace_config_name: "pnpm-workspace.yaml",
    workspace_config_field: "",
};

// const BUN_CONFIG: PackageManagerConfig = PackageManagerConfig {
//     manifest_name: "package.json",
//     lockfile_name: "bun.lock",
//     workspace_config_name: "",
//     workspace_config_field: "workspaces",
// };

// const YARN_CONFIG: PackageManagerConfig = PackageManagerConfig {
//     manifest_name: "package.json",
//     lockfile_name: "yarn.lock",
//     workspace_config_name: "",
//     workspace_config_field: "workspaces",
// };

#[derive(Debug, Clone, Copy)]
pub enum PackageManager {
    Npm,
    Pnpm,
    // Bun,
    // Yarn
}

#[derive(Debug)]
pub struct Script {
    pub name: String,
    pub command: String
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub path: PathBuf, // relative or absolute?
    pub scripts: Vec<Script>
}

#[derive(Debug)]
pub struct Workspace {
    pub root_path: PathBuf,
    pub package_manager: PackageManager,
    pub packages: Vec<Package>
}

pub struct Project {
    pub package_manager: PackageManager,
    pub root_path: PathBuf,
    pub config: PackageManagerConfig
}


pub const PACKAGE_MANAGERS: [PackageManager; 2] = [
    PackageManager::Npm,
    PackageManager::Pnpm,
];

#[derive(Debug, Deserialize)]
pub struct PackageJsonPartialSchema {
    pub name: Option<String>,
    pub workspaces: Option<Vec<String>>,
    pub scripts: Option<HashMap<String, String>>
}



#[derive(Debug, Deserialize)]
pub struct PnpmWorkspaceYamlPartialSchema {
    pub packages: Option<Vec<String>>,
}


// #[derive(Debug, Deserialize)]
// pub struct Importers {
//     #[serde(rename = ".")]
//     pub package_paths: HashMap<String, serde_yaml::Value>
// }

// #[derive(Debug, Deserialize)]
// pub struct PnpmLockfileYamlPartialSchema {
//     pub importers: Importers
// }

#[derive(Debug, Deserialize)]
pub struct PnpmLockfileYamlPartialSchema {
    pub importers: HashMap<String, serde_yaml::Value>
}
