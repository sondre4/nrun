use std::collections::HashSet;

use crate::{tui::{app::{App, Selection}, rendering::ui_rows::UiRow, }, workspace::types::Package};

fn is_package_expanded(package: &Package, expanded_set: &HashSet<usize>) -> bool {
    return expanded_set.contains(&package.id);
}

pub fn get_visible_rows(app: &App) -> Vec<UiRow> {
    let mut visible_rows = Vec::new();

    for package in &app.workspace.packages {
        let package_ui_row = UiRow::Package { package_id: package.id };
        visible_rows.push(package_ui_row);

        if is_package_expanded(package, &app.expanded_packages) {
            
            for script in &package.scripts {
                let script_ui_row = UiRow::Script { package_id: package.id, script_id: script.id };
                visible_rows.push(script_ui_row);
            }

            // add empty row
            visible_rows.push(UiRow::Vspace);
        }
    }

    return visible_rows;
}

pub fn find_index_of_selection_in_visible_rows(selection: &Selection, visible_rows: Vec<UiRow>) -> Option<usize> {
    let selected_package_id = selection.package_id;
    let selected_script_id = selection.script_id;

    let selected_row_wrapped = visible_rows.iter().position(|ui_row| {
        match ui_row {
            UiRow::Package { package_id } => {
                // package name selected
                if *package_id == selected_package_id && selected_script_id.is_none() {
                    return true;
                } else {
                    return false;
                }
            },

            UiRow::Script { package_id, script_id } => {
                // script name selected
                if selected_script_id.is_none() {
                    return false;
                }

                if *package_id == selected_package_id && *script_id == selected_script_id.unwrap() {
                    return true;
                } else {
                    return false;
                }
            },

            UiRow::Vspace => {
                return false;
            }
        }
    });

    if selected_row_wrapped.is_some() {
        return Some(selected_row_wrapped.unwrap());
    }

    return None;
}
