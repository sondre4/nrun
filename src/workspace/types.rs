use std::{fmt, path::PathBuf};

// for workspace metadata

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Bun,
    Yarn,
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageManager::Npm => write!(f, "npm"),
            PackageManager::Pnpm => write!(f, "pnpm"),
            PackageManager::Yarn => write!(f, "yarn"),
            PackageManager::Bun => write!(f, "bun"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script {
    pub id: usize,
    pub name: String,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub id: usize,
    pub name: String,
    pub path: PathBuf, // relative or absolute?
    pub scripts: Vec<Script>,
}

/// Domain type Workspace in the context of this tool refers to a project, which can be either a normal project or a monorepo
/// Some package managers refer to workspaces as the packages in a monorepo project.
/// In the context of this tool's domain types, a Package refers to the packages in a monorepo or the project root config
#[derive(Debug, PartialEq, Eq)]
pub struct Workspace {
    pub root_path: PathBuf,
    pub package_manager: PackageManager,
    pub packages: Vec<Package>,
}
