use std::path::PathBuf;

use nrun::workspace::{
    config::{NPM_CONFIG, Project},
    project::find::find_monorepo_project,
    types::PackageManager,
};
// use std::process::Command;

#[test]
fn finds_npm_project() {
    let base_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = base_directory.join("tests").join("fixtures").join("npm-project");

    // Wrong path argument
    let wrong_path_result = find_monorepo_project(base_directory, 5);
    assert!(wrong_path_result.is_err());

    // Too small depth argument
    let too_small_depth_result = find_monorepo_project(fixture_path.clone(), 0);
    assert!(too_small_depth_result.is_err());

    // Should be positive path
    let found_result = find_monorepo_project(fixture_path.clone(), 0);
    assert!(found_result.is_ok());

    // Check that the correct project was found

    let expected_project: Project = Project {
        package_manager: PackageManager::Npm,
        root_path: fixture_path,
        config: NPM_CONFIG,
    };

    let project = found_result.unwrap();
    assert_eq!(project, expected_project);
}
