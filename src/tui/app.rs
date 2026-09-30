use std::collections::HashSet;

use crate::{tui::{action::{Action, change_selection::{self, move_selection}, package_expansion::toggle_expansion}, operation::{Operation, action_to_operation}, rendering::{ui_rows::TuiRow}}, workspace::types::Workspace};

#[derive(Debug)]
pub struct Selection {
    // pub package_id: usize,
    // pub script_id: Option<usize>,
    pub row: TuiRow
}

pub enum SelectionMovement {
    Up,
    Down
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedSection {
    Main,
    Search
}

pub struct AppState {
    pub workspace: Workspace,
    pub expanded_packages: HashSet<usize>,
    pub selection: Selection,
    pub search_query: String,
    pub running: bool,
    pub focused_section: FocusedSection
}

pub struct TuiViewData {
    pub filtered_rows: Vec<TuiRow>,
    pub current_scroll_offset: usize
    // pub selected_row_index: usize
}

impl TuiViewData {
    fn default() -> TuiViewData {
        return TuiViewData {
            filtered_rows: Vec::new(),
            current_scroll_offset: 0
            // selected_row_index: 0
        };
    }
}

pub struct App {
    pub app_state: AppState,
    pub tui_view_data: TuiViewData
}

impl Selection {
    pub fn new() -> Selection {

        return Selection {
            row: TuiRow::Script { package_id: 0, script_id: 0 } // initialize to first script, change later?
            // row: {

            // },
            // package_id: 0,
            // script_id: None,
        };
    }
}

impl App {
    pub fn new(app_data: Workspace) -> App {
        let app_state: AppState = AppState {
            workspace: app_data,
            expanded_packages: HashSet::new(),
            selection: Selection::new(),
            search_query: String::new(),
            running: true,
            focused_section: FocusedSection::Main
        };

        let app: App = App {
            app_state: app_state,
            tui_view_data: TuiViewData::default()
        };

        return app;
    }

    pub fn initialize_expanded_packages(&mut self) -> () {
        self.app_state.expanded_packages.insert(self.app_state.workspace.packages[0].id);
    }

    pub fn action_to_operation(&mut self, action: Action) -> Operation {
        let operation = action_to_operation(self, action);
        return operation;
    }

    pub fn handle_operation(&mut self, operation: Operation) -> () {
        match operation {
            Operation::Nothing => { },

            // Exit program
            Operation::Quit => {
                self.app_state.running = false;
            },

            // Navigation
            Operation::MoveUp => {
                move_selection(self, SelectionMovement::Up);
            },
            Operation::MoveDown => {
                move_selection(self, SelectionMovement::Down);
            },

            // Interaction
            Operation::ConfirmSelection => todo!(),
            Operation::TogglePackageExpansion => {
                toggle_expansion(self);
            },

            // Searching
            Operation::StartSearch => todo!(),
            Operation::FinishSearch => todo!(),
            Operation::ExitSearch => todo!(),

            _ => { }
        }
    }
}
