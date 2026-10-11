use std::path::PathBuf;

use serde::Serialize;

use crate::workspace::types::{Package, PackageManager, Script, Workspace};

#[derive(Serialize)]
pub enum PackageManagerRepresentation {
    Npm,
    Pnpm,
    Bun,
    Yarn,
}

impl From<&PackageManager> for PackageManagerRepresentation {
    fn from(package_manager: &PackageManager) -> Self {
        match package_manager {
            PackageManager::Npm => return PackageManagerRepresentation::Npm,
            PackageManager::Pnpm => return PackageManagerRepresentation::Pnpm,
            PackageManager::Bun => return PackageManagerRepresentation::Bun,
            PackageManager::Yarn => return PackageManagerRepresentation::Yarn,
        }
    }
}

#[derive(Serialize)]
pub struct ScriptRepresentation {
    pub name: String,
    pub command: String,
}

impl From<&Script> for ScriptRepresentation {
    fn from(script: &Script) -> Self {
        return Self {
            name: script.name.clone(),
            command: script.command.clone(),
        };
    }
}

#[derive(Serialize)]
pub struct PackageRepresentation {
    pub name: String,
    pub path: PathBuf,
    pub scripts: Vec<ScriptRepresentation>,
}

impl From<&Package> for PackageRepresentation {
    fn from(package: &Package) -> Self {
        let mut representation: PackageRepresentation = PackageRepresentation {
            name: package.name.clone(),
            path: package.path.clone(),
            scripts: Vec::new(),
        };

        for script in &package.scripts {
            let script_representation = ScriptRepresentation::from(script);
            representation.scripts.push(script_representation);
        }

        return representation;
    }
}

#[derive(Serialize)]
pub struct WorkspaceRepresentation {
    pub root_path: PathBuf,
    pub package_manager: PackageManagerRepresentation,
    pub packages: Vec<PackageRepresentation>,
}

impl From<&Workspace> for WorkspaceRepresentation {
    fn from(workspace: &Workspace) -> Self {
        let mut representation: WorkspaceRepresentation = WorkspaceRepresentation {
            root_path: workspace.root_path.clone(),
            package_manager: PackageManagerRepresentation::from(&workspace.package_manager),
            packages: Vec::new(),
        };

        for package in &workspace.packages {
            let package_representation = PackageRepresentation::from(package);
            representation.packages.push(package_representation);
        }

        return representation;
    }
}
