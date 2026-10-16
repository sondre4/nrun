use std::path::{Path, PathBuf};

use nrun::{
    command::{command_caller::CommandCaller, script_config::ScriptConfig},
    workspace::find_workspace,
};

use crate::setup_temp_directory::{create_temp_project_from_base, get_cargo_manifest_dir_path};

fn test_can_call_project_script(project_path: &Path) -> () {
    let temp_project = create_temp_project_from_base(&get_cargo_manifest_dir_path(), project_path);
    let temp_project_path = temp_project.root_path().to_path_buf();

    println!("Temp project root path: {:#?}", temp_project.root_path());

    // Find workspace packages/scripts
    let found_workspace = find_workspace(Some(&temp_project_path), &5);
    if found_workspace.is_err() {
        println!("Error while finding workspace: {}", found_workspace.as_ref().err().unwrap());
    }

    assert!(found_workspace.is_ok());

    let workspace = found_workspace.unwrap();

    // Choose any script (project root in project fixtures have no scripts)
    let chosen_package = &workspace.packages[0]; // Only 1 package for a normal project => index 0
    let chosen_script = &chosen_package.scripts[0];
    let config_result = ScriptConfig::from_config_data(chosen_script, chosen_package, &workspace.package_manager);

    assert!(config_result.is_ok());

    let config = config_result.unwrap();

    let call_result = CommandCaller::call(config);
    assert!(call_result.is_ok());

    // Check if script in project fixture ran
    // The script should create a file called 'test.txt'
    let test_file_path = temp_project_path.join("test.txt");
    assert!(test_file_path.exists(), "File not found at {}", test_file_path.to_string_lossy());
}

#[test]
fn npm_project() -> () {
    test_can_call_project_script(&PathBuf::from("npm-project"));
}
