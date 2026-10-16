use std::path::{Path, PathBuf};

use nrun::workspace::{
    find_workspace,
    types::{Package, PackageManager, Script, Workspace},
};

fn test_discover_project_workspace(project_path: &Path, expected_package_manager: PackageManager, package_manager_name: String) -> () {
    let base_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = base_directory.join("tests").join("fixtures").join(project_path);

    // Wrong path argument
    let wrong_path_result = find_workspace(Some(&base_directory.clone()), &5);
    assert!(wrong_path_result.is_err());

    // Too small depth argument
    let too_small_depth_result = find_workspace(Some(&fixture_path.clone()), &0);
    assert!(too_small_depth_result.is_err());

    // Should be positive path
    let found_workspace = find_workspace(Some(&fixture_path.clone()), &5);
    if found_workspace.is_err() {
        println!("{}", found_workspace.as_ref().err().unwrap());
    }

    assert!(found_workspace.is_ok());

    // build expected data
    let expected_script = Script {
        id: 0,
        name: format!("run-{package_manager_name}-project"),
        command: "node index.js".to_string(),
    };

    let expected_scripts = vec![expected_script];

    let expected_package = Package {
        id: 0,
        name: format!("fixture-{package_manager_name}-project"),
        path: fixture_path.clone(),
        scripts: expected_scripts,
    };

    let expected_packages = vec![expected_package];

    let expected_workspace = Workspace {
        root_path: fixture_path,
        package_manager: expected_package_manager,
        packages: expected_packages,
    };

    let workspace = found_workspace.unwrap();
    assert_eq!(workspace, expected_workspace);
}

#[test]
fn npm_project() -> () {
    test_discover_project_workspace(&PathBuf::from("npm-project"), PackageManager::Npm, String::from("npm"));
}

#[test]
fn bun_project() -> () {
    test_discover_project_workspace(&PathBuf::from("bun-project"), PackageManager::Bun, String::from("bun"));
}
