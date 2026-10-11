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

#[derive(Debug, Clone)]
pub struct Script {
    pub id: usize,
    pub name: String,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub id: usize,
    pub name: String,
    pub path: PathBuf, // relative or absolute?
    pub scripts: Vec<Script>,
}

#[derive(Debug)]
pub struct Workspace {
    pub root_path: PathBuf,
    pub package_manager: PackageManager,
    pub packages: Vec<Package>,
}
