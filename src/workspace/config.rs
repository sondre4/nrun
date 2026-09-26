use std::path::PathBuf;

use crate::workspace::types::PackageManager;

pub struct MonorepoConfig {
    pub manifest_name: &'static str,
    pub lockfile_name: &'static str,
    pub workspace_config_name: &'static str,
    pub workspace_config_field: &'static str
}

pub const NPM_CONFIG: MonorepoConfig = MonorepoConfig {
    manifest_name: "package.json",
    lockfile_name: "package-lock.json",
    workspace_config_name: "",
    workspace_config_field: "workspaces"
};

pub const BUN_CONFIG: MonorepoConfig = MonorepoConfig {
    manifest_name: "package.json",
    lockfile_name: "bun.lock",
    workspace_config_name: "",
    workspace_config_field: "workspaces"
};

pub const YARN_CONFIG: MonorepoConfig = MonorepoConfig {
    manifest_name: "package.json",
    lockfile_name: "yarn.lock",
    workspace_config_name: "",
    workspace_config_field: "workspaces"
};

pub const PNPM_CONFIG: MonorepoConfig = MonorepoConfig {
    manifest_name: "package.json",
    lockfile_name: "pnpm-lock.yaml",
    workspace_config_name: "pnpm-workspace.yaml",
    workspace_config_field: ""
};


pub struct Project {
    pub package_manager: PackageManager,
    pub root_path: PathBuf,
    pub config: MonorepoConfig
}
