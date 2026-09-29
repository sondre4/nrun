use std::collections::HashSet;

use crate::{tui::{action::{Action, change_selection::{self, move_selection}, package_expansion::toggle_expansion}, operation::{Operation, action_to_operation}, rendering::ui_rows::TuiRow}, workspace::types::Workspace};

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
pub enum UiState {
    All,
    Main,
    Search
}

pub struct App {
    pub workspace: Workspace,
    pub expanded_packages: HashSet<usize>,
    pub selection: Selection,
    pub search_query: String,
    pub running: bool,
    pub ui_state: UiState
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
        let app: App = App {
            workspace: app_data,
            expanded_packages: HashSet::new(),
            selection: Selection::new(),
            search_query: String::new(),
            running: true,
            ui_state: UiState::Main
        };

        return app;
    }

    pub fn initialize_expanded_packages(&mut self) -> () {
        // for package in &self.workspace.packages {
        //     self.expanded_packages.insert(package.id);
        // }
        self.expanded_packages.insert(self.workspace.packages[0].id);
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
                self.running = false;
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
