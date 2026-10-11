use crate::tui::{
    app::{Selection, SelectionMovement},
    rendering::{tui_rows::TuiRow, visibility::find_index_of_selection_in_visible_rows},
};

fn get_new_index(old_index: usize, change: i64) -> usize {
    return (old_index as i64 + change) as usize;
}

fn get_selection_change(direction: &SelectionMovement) -> i64 {
    match direction {
        SelectionMovement::Up => {
            return -1;
        }
        SelectionMovement::Down => {
            return 1;
        }
    }
}

fn change_within_bounds(collection_length: usize, current_index: usize, change: i64) -> bool {
    let new_index = current_index as i64 + change;
    if new_index < 0 {
        return false;
    }

    if new_index >= collection_length as i64 {
        return false;
    }

    return true;
}

fn get_new_selection(rows: &Vec<TuiRow>, selected_row_index: usize, direction: &SelectionMovement) -> Option<Selection> {
    let row_collection_length = rows.len();
    let mut new_index = selected_row_index;

    loop {
        let change = get_selection_change(direction);
        if !change_within_bounds(row_collection_length, new_index, change) {
            return None;
        }

        new_index = get_new_index(new_index, change);

        if matches!(rows[new_index], TuiRow::Vspace) {
            continue;
        }
        break;
    }

    let new_row = &rows[new_index];
    let move_to_row: TuiRow = new_row.clone();

    let new_selection: Selection = Selection {
        row: move_to_row,
    };

    return Some(new_selection);
}

// app decides which selection is used, depending on which section is focused
pub fn get_new_selection_after_move(
    current_filtered_rows: &Vec<TuiRow>, current_selection: &Selection, direction: SelectionMovement,
) -> Option<Selection> {
    let selected_visible_row_index_wrapped = find_index_of_selection_in_visible_rows(current_selection, current_filtered_rows);

    if selected_visible_row_index_wrapped.is_none() {
        return None;
    }

    let current_index = selected_visible_row_index_wrapped.unwrap();
    let new_selection = get_new_selection(current_filtered_rows, current_index, &direction);

    if new_selection.is_some() {
        // app.app_state.selection = new_selection.unwrap();
        return Some(new_selection.unwrap());
    }

    return None;
}
