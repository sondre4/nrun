use std::path::PathBuf;

use nrun::workspace::{
    find_workspace,
    types::{Package, PackageManager, Script, Workspace},
};

#[test]
fn npm_monorepo() {
    let base_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path_base = base_directory.join("tests").join("fixtures").join("npm-monorepo");
    let fixture_path_package = base_directory.join("tests").join("fixtures").join("npm-monorepo").join("packages").join("app-a");

    // Wrong path argument
    let wrong_path_result = find_workspace(&Some(base_directory), &5);
    assert!(wrong_path_result.is_err());

    // Too small depth arguments
    let too_small_depth_result_1 = find_workspace(&Some(fixture_path_base.clone()), &0);
    assert!(too_small_depth_result_1.is_err());

    let too_small_depth_result_2 = find_workspace(&Some(fixture_path_package.clone()), &1);
    assert!(too_small_depth_result_2.is_err());

    // Should be positive path
    let found_workspace = find_workspace(&Some(fixture_path_package.clone()), &5);
    if found_workspace.is_err() {
        println!("Error while finding workspace: {}", found_workspace.as_ref().err().unwrap());
    }

    assert!(found_workspace.is_ok());

    let workspace = found_workspace.unwrap();

    // build expected data
    let expected_packages = vec![
        Package {
            id: 0,
            name: "fixture-npm-monorepo".to_string(),
            path: fixture_path_base.clone().join(""),
            scripts: vec![],
        },
        Package {
            id: 0,
            name: "fixture-app-a".to_string(),
            path: fixture_path_base.clone().join("packages").join("app-a"),
            scripts: vec![Script {
                id: 0,
                name: "run-a".to_string(),
                command: "node index.js".to_string(),
            }],
        },
        Package {
            id: 0,
            name: "fixture-app-b".to_string(),
            path: fixture_path_base.clone().join("packages").join("app-b"),
            scripts: vec![Script {
                id: 0,
                name: "run-b".to_string(),
                command: "node index.js".to_string(),
            }],
        },
    ];

    let expected_workspace = Workspace {
        root_path: fixture_path_base,
        package_manager: PackageManager::Npm,
        packages: expected_packages,
    };

    // Check expected workspace fields except packages
    assert_eq!(workspace.root_path, expected_workspace.root_path);
    assert_eq!(workspace.package_manager, expected_workspace.package_manager);

    let expected_package_length = expected_workspace.packages.len();

    for i in 0..expected_package_length {
        let expected_package = &expected_workspace.packages[i];

        let found_workspace_package = workspace.packages.iter().find(|p| {
            return p.name == expected_package.name;
        });

        assert!(found_workspace_package.is_some());
        let workspace_package = found_workspace_package.unwrap();

        // Found packages and expected packages have different value in 'id' field,
        // it is not needed to check it
        // assert_eq!(workspace_package, expected_package);
        assert_eq!(workspace_package.name, expected_package.name);
        assert_eq!(workspace_package.path, expected_package.path);
        assert_eq!(workspace_package.scripts, expected_package.scripts);
    }
}
