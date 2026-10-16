use std::path::{Path, PathBuf};

use nrun::workspace::{
    config::{NPM_CONFIG, Project},
    project::find::find_monorepo_project,
    types::PackageManager,
};

fn test_discover_project_info(project_path: &Path) -> () {
    let base_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = base_directory.join("tests").join("fixtures").join(project_path);

    // Wrong path argument
    let wrong_path_result = find_monorepo_project(base_directory, 5);
    assert!(wrong_path_result.is_err());
    // if wrong_path_result.is_ok() {
    //     return false;
    // }

    // Too small depth argument
    let too_small_depth_result = find_monorepo_project(fixture_path.clone(), 0);
    assert!(too_small_depth_result.is_err());

    // if too_small_depth_result.is_ok() {
    //     return false;
    // }

    // Should be positive path
    let found_result = find_monorepo_project(fixture_path.clone(), 5);
    if found_result.is_err() {
        println!("{}", found_result.as_ref().err().unwrap());
        // return false;
    }

    assert!(found_result.is_ok());

    // Check that the correct project was found
    let expected_project: Project = Project {
        package_manager: PackageManager::Npm,
        root_path: fixture_path,
        config: NPM_CONFIG,
    };

    let project = found_result.unwrap();
    // return project == expected_project;
    assert_eq!(project, expected_project);
}

#[test]
fn npm_project() -> () {
    // assert!(discovers_project(&PathBuf::from("npm-project")));
    test_discover_project_info(&PathBuf::from("npm-project"));
}
