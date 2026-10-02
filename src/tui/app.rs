use std::collections::HashSet;

use tui_input::Input;

use crate::{search::matching::search_query_empty, tui::{action::Action, operation::{Operation, OperationResult, action_to_operation, move_selection::move_selection}, rendering::tui_rows::TuiRow}, workspace::types::Workspace};

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
    expanded_packages: HashSet<usize>,
    selection: Selection,
    pub search_input: Input,
    pub running: bool,
    pub focused_section: FocusedSection
}

pub struct TuiViewData {
    pub filtered_rows: Vec<TuiRow>,
    search_expanded_packages: HashSet<usize>,
    search_package_selection: Selection,
    pub current_scroll_offset: usize
    // pub selected_row_index: usize
}

impl TuiViewData {
    fn default() -> TuiViewData {
        return TuiViewData {
            filtered_rows: Vec::new(),
            search_expanded_packages: HashSet::new(),
            search_package_selection: Selection::new(),
            current_scroll_offset: 0
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
        };
    }
}

impl App {
    pub fn new(app_data: Workspace) -> App {
        let app_state: AppState = AppState {
            workspace: app_data,
            expanded_packages: HashSet::new(),
            selection: Selection::new(),
            search_input: Input::default(),
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

    fn toggle_expansion(&mut self) -> OperationResult {
        let current_selection = self.get_current_selection();
        let selected_package_id_wrapped = current_selection.row.get_package_id();

        if selected_package_id_wrapped.is_none() {
            return OperationResult::None;
        }

        let selected_package_id = selected_package_id_wrapped.unwrap();

        let expanded_set = self.get_current_expanded_packages();

        if expanded_set.contains(&selected_package_id) {
            self.collapse_package(selected_package_id);
            return OperationResult::PackageCollapsed;
        } else {
            self.expand_package(selected_package_id);
            return OperationResult::PackageExpanded;
        }
    }

    fn currently_searching(&self) -> bool {
        // return self.app_state.search_input.value().len() > 0;
        let empty = search_query_empty(&self.app_state.search_input.value());
        return !empty;
    }

    // methods that are called during rendering
    pub fn search_focused(&self) -> bool {
        let search_focused = matches!(self.app_state.focused_section, FocusedSection::Search);
        return search_focused;
    }

    pub fn get_current_selection(&self) -> &Selection {
        if self.currently_searching() {
            return &self.tui_view_data.search_package_selection;
        }
        return &self.app_state.selection;
    }

    pub fn set_current_selection(&mut self, new_selection: Selection) -> () {
        if self.currently_searching() {
            self.tui_view_data.search_package_selection = new_selection;
        } else {
            self.app_state.selection = new_selection;
        }
    }

    pub fn get_current_expanded_packages(&self) -> &HashSet<usize> {
        if self.currently_searching() {
            return &self.tui_view_data.search_expanded_packages;
        }
        return &self.app_state.expanded_packages;
    }

    // methods that edit app state fields
    fn expand_package(&mut self, package_id: usize) -> () {
        if self.currently_searching() {
            self.tui_view_data.search_expanded_packages.insert(package_id);
        } else {
            self.app_state.expanded_packages.insert(package_id);
        }
    }

    fn collapse_package(&mut self, package_id: usize) -> () {
        if self.currently_searching() {
            self.tui_view_data.search_expanded_packages.remove(&package_id);
        } else {
            self.app_state.expanded_packages.remove(&package_id);
        }
    }

    fn reset_selection_on_search_update(&mut self) -> () {
        if !self.tui_view_data.filtered_rows.is_empty() {
            self.tui_view_data.search_package_selection.row = self.tui_view_data.filtered_rows[0].clone();
        }
    }

    fn fill_searching_expanded_packages(&mut self) -> () {
        for package in &self.app_state.workspace.packages {
            self.tui_view_data.search_expanded_packages.insert(package.id);
        }
    }

    fn reset_searching_expanded_packages(&mut self) -> () {
        self.tui_view_data.search_expanded_packages.clear();
    }

    pub fn handle_operation(&mut self, operation: Operation) -> OperationResult {
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
            Operation::ConfirmSelection => {
                // script has been selected
            },

            Operation::TogglePackageExpansion => {
                let operation_result = self.toggle_expansion();
                return operation_result;
            },

            // Searching
            Operation::FocusSearch => {
                self.app_state.focused_section = FocusedSection::Search;
            },

            // finish search and exit search are logically similar operations
            // while focused in search, both Enter and Esc will switch focus back to main
            Operation::FinishSearch => {
                self.app_state.focused_section = FocusedSection::Main;
            },

            Operation::CancelSearch => {
                self.app_state.search_input.reset();
                self.reset_searching_expanded_packages();
                // resetting the search selection when canceling search should not be needed
                // because the normal selection will be used 
                // and the search selection is reset on search input update anyway
                self.app_state.focused_section = FocusedSection::Main;
            },

            // tui-input handles text editing
            Operation::SearchOperation(input_request) => {
                self.reset_selection_on_search_update();
                self.fill_searching_expanded_packages();
                self.app_state.search_input.handle(input_request);
            },

            _ => { }
        }

        return OperationResult::None;
    }
}
