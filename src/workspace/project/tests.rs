use std::path::PathBuf;

use crate::workspace::{
    config::{BUN_CONFIG, NPM_CONFIG, PNPM_CONFIG, Project, YARN_CONFIG},
    project::find::build_project_info,
    types::PackageManager,
};

#[test]
fn builds_correct_project_data_when_npm() {
    let example_path: PathBuf = PathBuf::from("path/to/project");
    let package_manager: PackageManager = PackageManager::Npm;

    let expected_project: Project = Project {
        package_manager: PackageManager::Npm,
        root_path: example_path.clone(),
        config: NPM_CONFIG,
    };

    let built_data = build_project_info(example_path, package_manager);

    assert_eq!(built_data, expected_project);
}

#[test]
fn builds_correct_project_data_when_yarn() {
    let example_path: PathBuf = PathBuf::from("path/to/project");
    let package_manager: PackageManager = PackageManager::Yarn;

    let expected_project: Project = Project {
        package_manager: PackageManager::Yarn,
        root_path: example_path.clone(),
        config: YARN_CONFIG,
    };

    let built_data = build_project_info(example_path, package_manager);

    assert_eq!(built_data, expected_project);
}

#[test]
fn builds_correct_project_data_when_bun() {
    let example_path: PathBuf = PathBuf::from("path/to/project");
    let package_manager: PackageManager = PackageManager::Bun;

    let expected_project: Project = Project {
        package_manager: PackageManager::Bun,
        root_path: example_path.clone(),
        config: BUN_CONFIG,
    };

    let built_data = build_project_info(example_path, package_manager);

    assert_eq!(built_data, expected_project);
}

#[test]
fn builds_correct_project_data_when_pnpm() {
    let example_path: PathBuf = PathBuf::from("path/to/project");
    let package_manager: PackageManager = PackageManager::Pnpm;

    let expected_project: Project = Project {
        package_manager: PackageManager::Pnpm,
        root_path: example_path.clone(),
        config: PNPM_CONFIG,
    };

    let built_data = build_project_info(example_path, package_manager);

    assert_eq!(built_data, expected_project);
}
