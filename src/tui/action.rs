use crate::tui::{app::{App, SelectionMovement}, rendering::{ui_rows::UiRow, visibility::{find_index_of_selection_in_visible_rows, get_visible_rows}}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Nothing,
    Quit,
    Up,
    Down,
    StartSearch,
    ExitSearch,
    ExpandPackage,
    CollapsePackage,
    Confirm
}


// handle actions?

// fn move_selection_up(app: &mut App, )

pub fn move_selection(app: &mut App, direction: SelectionMovement) -> () {
    let currently_visible_rows = get_visible_rows(&app);
    let current_selection = &app.selection;

    let currently_selected_visible_index = find_index_of_selection_in_visible_rows(&current_selection, currently_visible_rows);

    match direction {
        SelectionMovement::Up => {

        },
        SelectionMovement::Down => {

        }
    }
}
