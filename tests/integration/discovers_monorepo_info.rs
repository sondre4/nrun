use std::path::{Path, PathBuf};

use nrun::workspace::{
    config::{BUN_CONFIG, NPM_CONFIG, PNPM_CONFIG, PackageManagerConfig, Project, YARN_CONFIG},
    project::find::find_monorepo_project,
    types::PackageManager,
};

fn test_discover_monorepo_info(monorepo_path: &Path, expected_package_manager: PackageManager, expected_config: PackageManagerConfig) -> () {
    let base_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path_base = base_directory.join("tests").join("fixtures").join(monorepo_path);
    let fixture_path_package = base_directory.join("tests").join("fixtures").join(monorepo_path).join("packages").join("package-a");

    // Wrong path argument
    let wrong_path_result = find_monorepo_project(base_directory, 5);
    assert!(wrong_path_result.is_err());

    // Too small depth arguments
    let too_small_depth_result_1 = find_monorepo_project(fixture_path_base.clone(), 0);
    assert!(too_small_depth_result_1.is_err());

    let too_small_depth_result_2 = find_monorepo_project(fixture_path_package.clone(), 1);
    assert!(too_small_depth_result_2.is_err());

    // Should be positive path (finding project root while working directory is in a package)
    let found_result = find_monorepo_project(fixture_path_package.clone(), 5);
    if found_result.is_err() {
        println!("{}", found_result.as_ref().err().unwrap());
    }

    assert!(found_result.is_ok());

    // Check that the correct project was found
    let expected_project: Project = Project {
        package_manager: expected_package_manager,
        root_path: fixture_path_base,
        config: expected_config,
    };

    let project = found_result.unwrap();
    assert_eq!(project, expected_project);
}

#[test]
fn npm_project() -> () {
    test_discover_monorepo_info(&PathBuf::from("npm-project"), PackageManager::Npm, NPM_CONFIG);
}

#[test]
fn bun_project() -> () {
    test_discover_monorepo_info(&PathBuf::from("bun-project"), PackageManager::Bun, BUN_CONFIG);
}

#[test]
fn yarn_project() -> () {
    test_discover_monorepo_info(&PathBuf::from("yarn-project"), PackageManager::Yarn, YARN_CONFIG);
}

#[test]
fn pnpm_project() -> () {
    test_discover_monorepo_info(&PathBuf::from("pnpm-project"), PackageManager::Pnpm, PNPM_CONFIG);
}
