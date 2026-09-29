use crate::tui::{app::{App, Selection, SelectionMovement}, rendering::{ui_rows::UiRow, visibility::{find_index_of_selection_in_visible_rows, build_visible_rows}}};

fn get_new_index(old_index: usize, change: i64) -> usize {
    return (old_index as i64 + change) as usize;
}

fn get_selection_change(direction: &SelectionMovement) -> i64 {
    match direction {
        SelectionMovement::Up => {
            return -1;
        },
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

fn get_new_selection(rows: Vec<UiRow>, selected_row_index: usize, direction: &SelectionMovement) -> Option<Selection> {
    let mut running = true;    
    let row_collection_length = rows.len();
    let mut new_index = selected_row_index;
    let mut new_package_id: Option<usize> = None;
    let mut new_script_id: Option<usize> = None;

    // println!("1");
    
    while running {
        let change = get_selection_change(direction);
        if !change_within_bounds(row_collection_length, new_index, change) {
            return None;
        }

        // println!("2");

        new_index = get_new_index(new_index, change);
        // println!("3");

        if matches!(rows[new_index], UiRow::Vspace) {
            continue;
        }

        // println!("4");

        // println!("Found row, moving...");

        running = false;
        break;
    }

    let new_row = &rows[new_index];
    match new_row {
        UiRow::Package { package_id } => {
            new_package_id = Some(*package_id);
        },
        UiRow::Script { package_id, script_id } => {
            new_package_id = Some(*package_id);
            new_script_id = Some(*script_id);
        },
        UiRow::Vspace => {
            return None;
        }
    }

    let new_selection: Selection = Selection {
        package_id: new_package_id.unwrap(),
        script_id: new_script_id,
    };

    return Some(new_selection);
}

pub fn move_selection(app: &mut App, direction: SelectionMovement) -> () {
    let visible_rows = build_visible_rows(&app);
    let selection = &app.selection;

    let selected_visible_row_index_wrapped = find_index_of_selection_in_visible_rows(&selection, &visible_rows);

    if selected_visible_row_index_wrapped.is_none() {
        return;
    }

    let current_index = selected_visible_row_index_wrapped.unwrap();
    let new_selection = get_new_selection(visible_rows, current_index, &direction);

    // println!("New selection: {new_selection:#?}");

    if new_selection.is_some() {
        app.selection = new_selection.unwrap();
    }
}
