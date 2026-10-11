use crate::cli::commands::GlobalArguments;
use crate::cli::display_json::display_json;
use crate::cli::display_packages::display_packages;
use crate::cli::json_representation::WorkspaceRepresentation;
use crate::cli::non_interactive::{handle_non_interactive_call, select_non_interactive};
use crate::search::searcher::PackageSearcher;
use crate::tui::launch_tui;
use crate::workspace::find_workspace;
use crate::workspace::types::Workspace;

fn handle_finding_workspace(global_arguments: &GlobalArguments) -> Option<Workspace> {
    let workspace_result = find_workspace(&global_arguments.workspace, &global_arguments.depth);
    match workspace_result {
        Ok(result) => {
            return Some(result);
        }
        Err(error) => {
            println!("nrun: Unable to find workspace: {error}");
            return None;
        }
    }
}

pub fn handle_list_packages(global_arguments: GlobalArguments, json_output_format: bool) -> () {
    let found_workspace = handle_finding_workspace(&global_arguments);

    if found_workspace.is_none() {
        return;
    }

    let workspace = found_workspace.unwrap();
    let packages = &workspace.packages;

    if json_output_format {
        let representation = WorkspaceRepresentation::from(&workspace);
        display_json(&representation);
        return;
    }

    // Display normally if format is not JSON
    display_packages(packages);
}

pub fn handle_cli_search(global_arguments: GlobalArguments, search_query: &String, debug: bool, json_output_format: bool) -> () {
    let found_workspace = handle_finding_workspace(&global_arguments);

    if found_workspace.is_none() {
        return;
    }

    let mut searcher: PackageSearcher = PackageSearcher::new();
    if debug {
        searcher.enable_debug();
    }

    let mut workspace = found_workspace.unwrap();
    let packages = &workspace.packages;
    let search_results = searcher.search(packages, search_query);

    if json_output_format {
        // Overwrite old packages with search results before writing to stdout
        workspace.packages = search_results;

        let representation = WorkspaceRepresentation::from(&workspace);
        display_json(&representation);
        return;
    }

    // Display normally if format is not JSON
    display_packages(&search_results);
}

pub fn handle_cli_default(global_arguments: GlobalArguments) -> () {
    let found_workspace = handle_finding_workspace(&global_arguments);

    if found_workspace.is_none() {
        return;
    }

    if select_non_interactive(&global_arguments) {
        let non_interactive_result = handle_non_interactive_call(found_workspace.unwrap(), global_arguments);
        match non_interactive_result {
            Ok(_result) => {
                return;
            }

            Err(error) => {
                println!("nrun: Error whilst running command: {error}");
                return;
            }
        };
    }

    let launch_tui_result = launch_tui(found_workspace.unwrap());
    match launch_tui_result {
        Ok(result) => result,
        Err(error) => {
            println!("nrun: Encountered an error whilst running TUI: {error}");
            return;
        }
    };
}
