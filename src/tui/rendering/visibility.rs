// TODO: add map for caching results from more expensive operations like build_visible_rows, find_index_of_selection_in_visible_rows could be improved with a map of TuiRow

use std::collections::HashSet;

use crate::{tui::{app::{App, Selection}, rendering::ui_rows::TuiRow, }, workspace::types::Package};

fn is_package_expanded(package: &Package, expanded_set: &HashSet<usize>) -> bool {
    return expanded_set.contains(&package.id);
}

pub fn build_visible_rows(app: &App) -> Vec<TuiRow> {
    let mut visible_rows = Vec::new();

    for package in &app.workspace.packages {
        let package_ui_row = TuiRow::Package { package_id: package.id };
        visible_rows.push(package_ui_row);

        if is_package_expanded(package, &app.expanded_packages) {
            
            for script in &package.scripts {
                let script_ui_row = TuiRow::Script { package_id: package.id, script_id: script.id };
                visible_rows.push(script_ui_row);
            }

            // add empty row
            visible_rows.push(TuiRow::Vspace);
        }
    }

    return visible_rows;
}

pub fn find_index_of_selection_in_visible_rows(selection: &Selection, visible_rows: &Vec<TuiRow>) -> Option<usize> {
    let selected_row_wrapped = visible_rows.iter().position(|tui_row| {
        if *tui_row == selection.row {
            return true;
        }

        return false;
    });

    if selected_row_wrapped.is_some() {
        return Some(selected_row_wrapped.unwrap());
    }

    return None;
}
