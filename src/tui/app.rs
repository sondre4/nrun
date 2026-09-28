use std::collections::HashSet;

use crate::{tui::{action::{Action, change_selection::{self, move_selection}}, rendering::ui_rows::UiRow}, workspace::types::Workspace};

#[derive(Debug)]
pub struct Selection {
    pub package_id: usize,
    pub script_id: Option<usize>,
    // pub row: UiRow
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

pub struct Tui {

}


impl Selection {
    pub fn new() -> Selection {
        return Selection {
            package_id: 0,
            script_id: None,
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

    pub fn handle_action(&mut self, action: Action) -> () {
        match action {

            // selection movement
            Action::Up => {
                move_selection(self, SelectionMovement::Up);
            }
            Action::Down => {
                move_selection(self, SelectionMovement::Down);
            }

            // exit the program
            Action::Quit => {
                self.running = false;
            }

            _ => ()
        }
    }
}

// call handle action in action.rs?

// pub fn handle_action(app: &mut App, action: Action) -> () {
//     match action {
//         Action::Quit => {
//             app.running = false;
//         }

//         _ => ()
//     }
// }