use std::path::{Path, PathBuf};

use nrun::{
    command::{command_caller::CommandCaller, script_config::ScriptConfig},
    workspace::find_workspace,
};

use crate::setup_temp_directory::{create_temp_project_from_base, get_cargo_manifest_dir_path};

fn test_can_call_monorepo_script(monorepo_path: &Path) {
    const TEST_PACKAGE_NAME: &str = "app-a";

    let temp_monorepo = create_temp_project_from_base(&get_cargo_manifest_dir_path(), monorepo_path);
    let temp_monorepo_package_path = temp_monorepo.root_path().join("packages").join(TEST_PACKAGE_NAME);

    // Find workspace packages/scripts
    let found_workspace = find_workspace(Some(&temp_monorepo_package_path), &5);
    if found_workspace.is_err() {
        println!("Error while finding workspace: {}", found_workspace.as_ref().err().unwrap());
    }

    assert!(found_workspace.is_ok());

    let workspace = found_workspace.unwrap();

    // Choose any script (project root in monorepo fixtures have no scripts, find package 'app-a')
    let found_package = &workspace.packages.iter().find(|p| p.name == format!("fixture-{TEST_PACKAGE_NAME}"));
    assert!(found_package.is_some());
    let chosen_package = found_package.unwrap();

    let chosen_script = &chosen_package.scripts[0];
    let config_result = ScriptConfig::from_config_data(chosen_script, chosen_package, &workspace.package_manager);

    // This will fail if the package manager corresponding to the fixture project does not exist in the test environment
    assert!(config_result.is_ok());

    let config = config_result.unwrap();

    let call_result = CommandCaller::call(config);
    assert!(call_result.is_ok());

    // Check if script in monorepo fixture ran
    // The script should create a file called 'test.txt'
    let test_file_path = temp_monorepo_package_path.join("test.txt");
    assert!(test_file_path.exists(), "File not found at {}", test_file_path.to_string_lossy());
}

#[test]
fn npm_monorepo() -> () {
    test_can_call_monorepo_script(&PathBuf::from("npm-monorepo"));
}
