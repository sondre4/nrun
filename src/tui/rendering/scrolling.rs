use std::collections::HashSet;

use crate::{tui::{app::App, rendering::ui_rows::UiRow, }, workspace::types::Package};

fn is_package_expanded(package: &Package, expanded_set: &HashSet<usize>) -> bool {
    return expanded_set.contains(&package.id);
}

pub fn get_visible_rows(app: &App) -> Vec<UiRow> {
    let mut visible_rows = Vec::new();

    for package in &app.workspace.packages {
        let package_ui_row = UiRow::Package { id: package.id };
        visible_rows.push(package_ui_row);

        if is_package_expanded(package, &app.expanded_packages) {
            
            for script in &package.scripts {
                let script_ui_row = UiRow::Script { package_id: package.id, id: script.id };
                visible_rows.push(script_ui_row);
            }

            // add empty row
            visible_rows.push(UiRow::Vspace);
        }
    }

    return visible_rows;
}
