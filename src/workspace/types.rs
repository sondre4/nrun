use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

// for workspace metadata

#[derive(Debug, Clone, Copy)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Bun,
    Yarn
}


#[derive(Debug, Clone)]
pub struct Script {
    pub id: usize,
    pub name: String,
    pub command: String
}

#[derive(Debug, Clone)]
pub struct Package {
    pub id: usize,
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
