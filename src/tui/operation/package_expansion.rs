use std::cmp::{max, min};

use crate::tui::{rendering::tui_rows::TuiRow};

// pub fn toggle_expansion(app: &mut App) -> OperationResult {
//     let current_selection = app.get_current_selection();
//     let selected_package_id_wrapped = current_selection.row.get_package_id();

//     if selected_package_id_wrapped.is_none() {
//         return OperationResult::None;
//     }

//     let selected_package_id = selected_package_id_wrapped.unwrap();

//     if app.app_state.expanded_packages.contains(&selected_package_id) {
//         app.app_state.expanded_packages.remove(&selected_package_id);
//         return OperationResult::PackageCollapsed;
//     } else {
//         app.app_state.expanded_packages.insert(selected_package_id);
//         return OperationResult::PackageExpanded;
//     }
// }

pub fn last_index_of_script_in_package(filtered_rows: &Vec<TuiRow>, selected_row_index: usize) -> usize {
    let start = selected_row_index + 1; // +1, otherwise enters package immediately in the match statement (matches the selected row (package))
    let end_index = filtered_rows.len() - 1;
    let mut counter: usize = 0;

    for i in start..end_index {
        let row = &filtered_rows[i];

        match row {
            // found package row after selected row => means counter is amount of non-package rows
            TuiRow::Package { package_id: _ } => {
                break;
            },

            // if not package row, add 1
            _ => {
                counter += 1;
                
            }
        }
    }

    let last_index =start.saturating_add(counter);
    return min(last_index, end_index);
}

pub fn new_scroll_offset_after_expanding(filtered_rows: &Vec<TuiRow>, selected_row: &TuiRow, selected_row_index: usize, scroll_offset: usize, viewport_height: usize) -> usize {
    match selected_row {
        TuiRow::Package { package_id: _ } => {
            let last_script_index = last_index_of_script_in_package(filtered_rows, selected_row_index);

            if viewport_height == 0 {
                return scroll_offset;
            }

            let new_scroll_offset = last_script_index.saturating_sub(viewport_height - 1);
            return max(scroll_offset, new_scroll_offset);
        }

        // if selected row is not a package row (should not be possible), return original value
        _ => {
            return scroll_offset;
        }
    }
}
