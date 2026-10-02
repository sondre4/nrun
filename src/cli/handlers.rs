use crate::cli::commands::GlobalArguments;
use crate::cli::display_packages::display_packages;
use crate::search::PackageSearcher;
use crate::tui::launch_tui;
use crate::workspace::find_workspace;
use crate::workspace::types::Workspace;

fn handle_finding_workspace(global_arguments: GlobalArguments) -> Option<Workspace> {
    let workspace_result = find_workspace(global_arguments.path, global_arguments.depth);
    match workspace_result {
        Ok(result) => {
            return Some(result);
        },
        Err(error) => {
            println!("nrun: Unable to find workspace: {error}");
            return None;
        }
    }
}


pub fn handle_launching_tui(global_arguments: GlobalArguments) -> () {
    let found_workspace = handle_finding_workspace(global_arguments);

    if found_workspace.is_none() {
        return;
    }

    let launch_tui_result = launch_tui(found_workspace.unwrap());
    match launch_tui_result {
        Ok(result) => result,
        Err(error) => {
            println!("nrun: Error whilst launching tui: {error}");
            return;
        }
    };
}

pub fn handle_cli_search(global_arguments: GlobalArguments, search_query: &String) -> () {
    let found_workspace = handle_finding_workspace(global_arguments);

    if found_workspace.is_none() {
        return;
    }

    let mut searcher: PackageSearcher = PackageSearcher::new();
    let packages = found_workspace.unwrap().packages;

    let search_results = searcher.search(&packages, search_query);

}
