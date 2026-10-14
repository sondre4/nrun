use std::path::PathBuf;

use nrun::workspace::{
    config::{NPM_CONFIG, Project},
    project::find::find_monorepo_project,
    types::PackageManager,
};
// use std::process::Command;

#[test]
fn finds_npm_monorepo() {
    let base_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path_base = base_directory.join("tests").join("fixtures").join("npm-monorepo");
    let fixture_path_package = base_directory.join("tests").join("fixtures").join("npm-monorepo").join("packages").join("package-a");

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
        package_manager: PackageManager::Npm,
        root_path: fixture_path_base,
        config: NPM_CONFIG,
    };

    let project = found_result.unwrap();
    assert_eq!(project, expected_project);
}
