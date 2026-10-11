use crate::{
    cli::commands::GlobalArguments,
    command::{command_caller::CommandCaller, script_config::ScriptConfig},
    workspace::types::Workspace,
};

pub fn select_non_interactive(global_arguments: &GlobalArguments) -> bool {
    if global_arguments.package.is_some() && global_arguments.script.is_some() {
        return true;
    }

    return false;
}

pub fn handle_non_interactive_call(workspace: Workspace, global_arguments: GlobalArguments) -> Result<(), String> {
    if global_arguments.package.is_none() {
        return Err(format!("Argument 'package' is not present"));
    }

    if global_arguments.script.is_none() {
        return Err(format!("Argument 'script' is not present"));
    }

    let package_name = global_arguments.package.unwrap();
    let script_name = global_arguments.script.unwrap();

    let found_package = workspace.packages.iter().find(|p| {
        return p.name == package_name;
    });

    if found_package.is_none() {
        return Err(format!("Could not find package name '{package_name}' in the current workspace"));
    }

    let package = found_package.unwrap();

    let found_script = package.scripts.iter().find(|s| {
        return s.name == script_name;
    });

    if found_script.is_none() {
        return Err(format!("Could not find script name '{script_name}' in the current workspace"));
    }

    let script = found_script.unwrap();

    // Found package and script: call script
    let config = ScriptConfig::from_config_data(script, package, &workspace.package_manager)?;

    CommandCaller::call(config)?;
    return Ok(());
}
