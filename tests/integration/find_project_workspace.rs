use std::path::PathBuf;

use nrun::workspace::{
    config::{NPM_CONFIG, Project},
    // project::find::find_monorepo_project,
    types::PackageManager,
    find_workspace
};
// use std::process::Command;

#[test]
fn finds_npm_workspace() {
    let base_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = base_directory.join("tests").join("fixtures").join("npm-project");

    // Wrong path argument
    let wrong_path_result = find_workspace(base_directory, 5);
    assert!(wrong_path_result.is_err());

    // Too small depth argument
    let too_small_depth_result = find_workspace(fixture_path.clone(), 0);
    assert!(too_small_depth_result.is_err());

    // Should be positive path
    let found_workspace = find_workspace(fixture_path.clone(), 5);
    if found_workspace.is_err() {
        println!("{}", found_workspace.as_ref().err().unwrap());
    }

    assert!(found_workspace.is_ok());

    // Check that the correct project was found
    let expected_project: Project = Project {
        package_manager: PackageManager::Npm,
        root_path: fixture_path,
        config: NPM_CONFIG,
    };

    pub id: usize,
    pub name: String,
    pub command: String,

    let expected_script = Script {
        id: 0,
        name: "run-npm-project",
        command: "node index.js"
    };

    let expected_scripts = vec![expected_script];

    let expected_package = Package {
        id: 0,
        name: "fixture-npm-project",
        path: fixture_path.clone(),
        scripts: expected_scripts
    };

    let expected_packages = vec![expected_package];

    let expected_workspace = Workspace {
        root_path: fixture_path,
        package_manager: PackageManager::Npm,
        packages: expected_packages
    }

    let workspace = found_result.unwrap();
    assert_eq!(workspace, expected_workspace);
}
