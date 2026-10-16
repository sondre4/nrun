use std::path::{Path, PathBuf};

use nrun::{
    command::{command_caller::CommandCaller, script_config::ScriptConfig},
    workspace::find_workspace,
};

use crate::setup_temp_directory::create_temp_directory;

fn test_can_call_monorepo_script(monorepo_path: &Path) {
    let base_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_base_path = base_directory.join("tests").join("fixtures").join(monorepo_path);

    let temp_fixture_path_result = create_temp_directory(&fixture_base_path);
    assert!(temp_fixture_path_result.is_ok());
    let temp_fixture_directory = temp_fixture_path_result.unwrap();
    let temp_fixture_monorepo_root = temp_fixture_directory.path();

    // Check that temporary directory was created properly
    assert!(temp_fixture_monorepo_root.join("package.json").exists(), "Could not find package.json at '{}'", temp_fixture_monorepo_root.join("package.json").to_string_lossy());

    let temp_fixture_monorepo_package_path = temp_fixture_monorepo_root.join("packages").join("app-a");

    println!("{temp_fixture_monorepo_package_path:#?}");

    // Find workspace packages/scripts
    let found_workspace = find_workspace(Some(&temp_fixture_monorepo_package_path), &5);
    if found_workspace.is_err() {
        println!("Error while finding workspace: {}", found_workspace.as_ref().err().unwrap());
    }

    assert!(found_workspace.is_ok());

    let workspace = found_workspace.unwrap();

    // Choose any script (project root in npm monorepo fixtures have no scripts)
    let chosen_package = &workspace.packages[1];
    let chosen_script = &chosen_package.scripts[0];
    let config_result = ScriptConfig::from_config_data(chosen_script, chosen_package, &workspace.package_manager);

    assert!(config_result.is_ok());

    let config = config_result.unwrap();

    let call_result = CommandCaller::call(config);
    assert!(call_result.is_ok());

    // Check if script in monorepo fixture ran.
    // The script should create a file called 'test.txt'
    let test_file_path = temp_fixture_monorepo_package_path.join("test.txt");
    assert!(test_file_path.exists(), "File not found at {}", test_file_path.to_string_lossy());
}

#[test]
fn npm_monorepo() -> () {
    test_can_call_monorepo_script(&PathBuf::from("npm-monorepo"));
}
