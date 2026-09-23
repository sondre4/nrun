use std::path::PathBuf;

pub struct PackageManagerConfig {
    pub manifest_name: &'static str,
    pub lockfile_name: &'static str,
    pub workspace_config_name: &'static str,
    pub workspace_config_field: &'static str
}

pub const NPM_CONFIG: PackageManagerConfig = PackageManagerConfig {
    manifest_name: "package.json",
    lockfile_name: "pnpm-lock.yaml",
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
//     lockfile_name: "pnpm-lock.yaml",
//     workspace_config_name: "",
//     workspace_config_field: "workspaces",
// };

// const YARN_CONFIG: PackageManagerConfig = PackageManagerConfig {
//     manifest_name: "package.json",
//     lockfile_name: "yarn.lock",
//     workspace_config_name: "",
//     workspace_config_field: "workspaces",
// };

#[derive(Clone, Copy)]
pub enum PackageManager {
    Npm,
    Pnpm,
    // Bun,
    // Yarn
}

pub struct Script {
    pub name: String,
    pub command: String
}

pub struct Package {
    pub name: String,
    pub path: PathBuf, // relative or absolute?
    pub scripts: Vec<Script>
}

pub struct Workspace {
    pub root_path: PathBuf,
    pub package_manager: PackageManager,
    pub packages: Vec<Script>
}

pub struct Project {
    pub package_manager: PackageManager,
    pub root_path: PathBuf
}


pub const PACKAGE_MANAGERS: [PackageManager; 2] = [
    PackageManager::Npm,
    PackageManager::Pnpm,
];
