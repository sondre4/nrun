use crate::tui::{app::{App, Selection, SelectionMovement}, rendering::{ui_rows::UiRow, visibility::{find_index_of_selection_in_visible_rows, get_visible_rows}}};

pub mod change_selection;
pub mod package_expansion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Nothing,
    Quit,
    Up,
    Down,
    StartSearch,
    ExitSearch,
    TogglePackageExpansion,
    Confirm
}
