use std::collections::HashSet;

use crate::{tui::keybinds::Action, workspace::types::Workspace};

pub struct Selection {
    pub visible_package_index: usize,
    pub visible_script_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiState {
    All,
    View,
    Search
}

pub struct App {
    pub workspace: Workspace,
    pub expanded_packages: HashSet<usize>,
    pub selected: Selection,
    pub search_query: String,
    pub running: bool,
    pub ui_state: UiState
}

pub struct Tui {

}


impl Selection {
    pub fn new() -> Selection {
        return Selection {
            visible_package_index: 0,
            visible_script_index: 0,
        };
    }
}

impl App {
    pub fn new(app_data: Workspace) -> App {
        let app: App = App {
            workspace: app_data,
            expanded_packages: HashSet::new(),
            selected: Selection::new(),
            search_query: String::new(),
            running: true,
            ui_state: UiState::View
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