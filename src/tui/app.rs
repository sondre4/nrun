use std::collections::HashSet;

use tui_input::Input;

use crate::{command::{command_caller::CommandCaller, script_config::ScriptConfig}, search::{matching::search_query_empty, searcher::PackageSearcher}, tui::{action::Action, operation::{Operation, OperationResult, action_to_operation, move_selection::get_new_selection_after_move}, rendering::{tui_rows::TuiRow, visibility::build_tui_rows}, styles::{colors::{ColorPalette, detect_color_capability}, ui_style::UiStyles}}, workspace::types::{Package, Script, Workspace}};

#[derive(Debug, Clone)]
pub struct Selection {
    pub row: TuiRow
}

impl Selection {
    pub fn new() -> Selection {
        return Selection {
            row: TuiRow::Script { package_id: 0, script_id: 0 } // initialize to first script, change later?
        };
    }
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
    workspace: Workspace,
    search_input: Input,
    running: bool,
    focused_section: FocusedSection,
    color_palette: ColorPalette,
    styles: UiStyles
}

#[derive(Clone)]
pub struct ViewData {
    pub expanded_set: HashSet<usize>,
    pub selection: Selection,
    pub scroll_offset: usize
}

impl ViewData {
    pub fn new() -> ViewData {
        return ViewData {
            expanded_set: HashSet::new(),
            selection:  Selection::new(),
            scroll_offset: 0
        };
    }
}

pub struct TuiViewData {
    filtered_rows: Vec<TuiRow>,
    main_view_data: ViewData,
    searching_view_data: ViewData
}

impl TuiViewData {
    fn default() -> TuiViewData {
        return TuiViewData {
            filtered_rows: Vec::new(),
            main_view_data: ViewData::new(),
            searching_view_data: ViewData::new()
        };
    }
}

pub struct App {
    app_state: AppState,
    tui_view_data: TuiViewData,
    searcher: PackageSearcher
}

impl App {
    pub fn new(app_data: Workspace) -> App {

        // initialize the style
        let color_capability = detect_color_capability();
        let palette = ColorPalette::from_capability(color_capability);
        let ui_styles = UiStyles::from_palette(&palette);

        let app_state: AppState = AppState {
            workspace: app_data,
            search_input: Input::default(),
            running: true,
            focused_section: FocusedSection::Main,
            color_palette: palette,
            styles: ui_styles
        };

        // Only created once
        let app: App = App {
            app_state: app_state,
            tui_view_data: TuiViewData::default(),
            searcher: PackageSearcher::new()
        };

        return app;
    }

    pub fn get_styles(&self) -> &UiStyles {
        return &self.app_state.styles;
    }

    pub fn get_color_palette(&self) -> &ColorPalette {
        return &self.app_state.color_palette;
    }

    pub fn initialize_expanded_packages(&mut self) -> () {
        self.tui_view_data.main_view_data.expanded_set.insert(self.app_state.workspace.packages[0].id);
    }

    pub fn running(&self) -> bool {
        return self.app_state.running;
    }


    pub fn find_package_by_id(&self, package_id: usize) -> Option<Package> {
        let found_package = self.app_state.workspace.packages.iter().find(|p| {
            p.id == package_id
        });

        return Some(found_package.unwrap().clone());
    }

    pub fn find_script_by_id(&self, package: Package, script_id: usize) -> Option<Script> {
        let found_script = package.scripts.iter().find(|s| {
            s.id == script_id
        });

        if found_script.is_none() {
            return None;
        }

        return Some(found_script.unwrap().clone());
    }

    pub fn get_focused_section(&self) -> &FocusedSection {
        return &self.app_state.focused_section;
    }


    fn currently_searching(&self) -> bool {
        let empty = search_query_empty(&self.app_state.search_input.value());
        return !empty;
    }

    pub fn get_search_input(&self) -> &Input {
        return &self.app_state.search_input;
    }


    // View data
    pub fn get_current_view_data(&self) -> ViewData {
        if self.currently_searching() {
            return self.tui_view_data.searching_view_data.clone();
        } else {
            return self.tui_view_data.main_view_data.clone();
        }
    }

    fn get_current_view_data_mut(&mut self) -> &mut ViewData {
        if self.currently_searching() {
            return &mut self.tui_view_data.searching_view_data;
        } else {
            return &mut self.tui_view_data.main_view_data;
        }
    }


    // Package expansion/collapsing
    fn expand_package(&mut self, package_id: usize) -> () {
        let current_view_data = self.get_current_view_data_mut();
        current_view_data.expanded_set.insert(package_id);
    }

    fn collapse_package(&mut self, package_id: usize) -> () {
        let current_view_data = self.get_current_view_data_mut();
        current_view_data.expanded_set.remove(&package_id);
    }

    fn toggle_expansion(&mut self) -> OperationResult {
        let current_view_data = self.get_current_view_data();
        let selected_package_id_wrapped = current_view_data.selection.row.get_package_id();

        if selected_package_id_wrapped.is_none() {
            return OperationResult::None;
        }

        let selected_package_id = selected_package_id_wrapped.unwrap();
        let expanded_set = &current_view_data.expanded_set;

        if expanded_set.contains(&selected_package_id) {
            self.collapse_package(selected_package_id);
            return OperationResult::PackageCollapsed;
        } else {
            self.expand_package(selected_package_id);
            return OperationResult::PackageExpanded;
        }
    }

    fn fill_searching_expanded_packages(&mut self) -> () {
        for package in &self.app_state.workspace.packages {
            self.tui_view_data.searching_view_data.expanded_set.insert(package.id);
        }
    }


    // Selection setters
    pub fn set_current_selection(&mut self, new_selection: Selection) -> () {
        let current_view_data = self.get_current_view_data_mut();
        current_view_data.selection = new_selection;
    }

    pub fn prepare_searching_expanding_packages_if_needed(&mut self, result: &OperationResult) -> () {
        let needs_updating = matches!(result, OperationResult::UpdatedSearchQuery);

        if needs_updating && self.currently_searching() {
            // All packages in search results are shown as expanded initially (search results are a subset of all packages)
            self.fill_searching_expanded_packages();
        }
    }

    pub fn prepare_current_selection_if_needed(&mut self, result: &OperationResult) -> () {
        let needs_updating = matches!(result, OperationResult::UpdatedSearchQuery);
        let enough_results = self.tui_view_data.filtered_rows.len() >= 2;

        // Only after updating search query and there are at least 2 rows
        if needs_updating && self.currently_searching() && enough_results {
            let new_selected_row_after_searching = self.tui_view_data.filtered_rows[1].clone();
            let new_selection_after_searching: Selection = Selection {
                row: new_selected_row_after_searching
            };

            self.set_current_selection(new_selection_after_searching);
        }
    }


    // Scroll offset
    pub fn update_current_scroll_offset(&mut self, new_scroll_offset: usize) -> () {
        let current_view_data = self.get_current_view_data_mut();
        current_view_data.scroll_offset = new_scroll_offset;
    }


    // methods that are called during rendering
    pub fn search_focused(&self) -> bool {
        let search_focused = matches!(self.app_state.focused_section, FocusedSection::Search);
        return search_focused;
    }


    // filtered rows
    // self.update_filtered_rows uses this setter
    fn set_filtered_rows(&mut self, new_filtered_rows: Vec<TuiRow>) -> () {
        self.tui_view_data.filtered_rows = new_filtered_rows;
    }

    pub fn get_filtered_rows(&self) -> Vec<TuiRow> {
        return self.tui_view_data.filtered_rows.clone();
    }

    pub fn update_filtered_rows(&mut self) -> () {
        if self.currently_searching() {
            let a = 5;
        }
        let original_packages = &self.app_state.workspace.packages;
        let search_results = self.searcher.search(original_packages, self.app_state.search_input.value());
        let current_expanded_packages = &self.get_current_view_data().expanded_set;
        let all_filtered_rows = build_tui_rows(&search_results, &current_expanded_packages);

        self.set_filtered_rows(all_filtered_rows);
    }


    // Action/operation
    pub fn action_to_operation(&mut self, action: Action) -> Operation {
        let operation = action_to_operation(self, action);
        return operation;
    }

    pub fn handle_operation(&mut self, operation: Operation) -> OperationResult {
        match operation {
            Operation::Nothing => { },

            // Exit program
            Operation::Quit => {
                self.app_state.running = false;
            },

            // Navigation
            // Navigation is an event, this event does not need to update search + filtered rows,
            // because those are updated in typing events
            Operation::MoveUp => {
                let current_view_data = self.get_current_view_data();
                let new_selection = get_new_selection_after_move(
                    &self.tui_view_data.filtered_rows,
                    &current_view_data.selection,
                    SelectionMovement::Up
                );

                if new_selection.is_some() {
                    self.set_current_selection(new_selection.unwrap());
                }
            },

            Operation::MoveDown => {
                let current_view_data = self.get_current_view_data();
                let new_selection = get_new_selection_after_move(
                    &self.tui_view_data.filtered_rows,
                    &current_view_data.selection,
                    SelectionMovement::Down
                );

                if new_selection.is_some() {
                    self.set_current_selection(new_selection.unwrap());
                }
            },

            // Interaction
            Operation::ConfirmSelection => {
                // script has been selected
                self.app_state.running = false;
                return OperationResult::ScriptCalled;
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
                // self.reset_searching_expanded_packages();
                // resetting the search selection when canceling search should not be needed
                // because the normal selection will be used 
                // and the search selection is reset on search input update anyway
                self.app_state.focused_section = FocusedSection::Main;
            },

            // tui-input handles text editing
            Operation::SearchOperation(input_request) => {
                self.app_state.search_input.handle(input_request);
                return OperationResult::UpdatedSearchQuery;
            },

            _ => { }
        }

        return OperationResult::None;
    }

    pub fn handle_operation_result(&self, result: &OperationResult) -> Result<(), String> {
        match result {
            OperationResult::ScriptCalled => {
                let current_view_data = self.get_current_view_data();
                let found_package_id = current_view_data.selection.row.get_package_id();
                let found_script_id = current_view_data.selection.row.get_script_id();

                if found_package_id.is_none() || found_script_id.is_none() {
                    return Err(format!("Missing package or script id"));
                }

                let found_package = self.find_package_by_id(found_package_id.unwrap());

                if found_package.is_none() {
                    return Err(format!("Could not find package"));
                }

                let package = found_package.unwrap();
                let package_path = package.path.clone();
                let found_script = self.find_script_by_id(package, found_script_id.unwrap());

                if found_script.is_none() {
                    return Err(format!("Could not find script"));
                }

                let config = ScriptConfig::from_config_data(
                    &found_script.unwrap(),
                    &package_path,
                    &self.app_state.workspace.package_manager
                )?;

                // println!("{config:#?}");

                CommandCaller::call(config)?;
                return Ok(());
            },

            _ => return Ok(())
        }
    }
}
