// TODO: add map for caching results from more expensive operations like build_visible_rows, find_index_of_selection_in_visible_rows could be improved with a map of TuiRow

use std::collections::HashSet;

use crate::{tui::{app::{App, Selection}, rendering::ui_rows::TuiRow, scrolling::TuiScrollState, }, workspace::types::Package};

fn is_package_expanded(package: &Package, expanded_set: &HashSet<usize>) -> bool {
    return expanded_set.contains(&package.id);
}

pub fn build_tui_rows(app: &App) -> Vec<TuiRow> {
    let mut tui_rows = Vec::new();

    for package in &app.workspace.packages {
        let package_ui_row = TuiRow::Package { package_id: package.id };
        tui_rows.push(package_ui_row);

        if is_package_expanded(package, &app.expanded_packages) {
            
            for script in &package.scripts {
                let script_ui_row = TuiRow::Script { package_id: package.id, script_id: script.id };
                tui_rows.push(script_ui_row);
            }

            // add empty row
            tui_rows.push(TuiRow::Vspace);
        }
    }

    return tui_rows;
}

pub fn to_visible_rows(possible_tui_rows: Vec<TuiRow>, scroll_state: &TuiScrollState) -> Vec<TuiRow> {
    let start = scroll_state.top_row_index;
    let end = start + scroll_state.available_space;

    // This creates a new Vec<T> containing elements from index a (inclusive) to b (exclusive)
    let visible_rows = possible_tui_rows[start..end].to_vec();
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
