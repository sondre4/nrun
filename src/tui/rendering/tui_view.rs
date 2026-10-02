use ratatui::{Frame, layout::Rect};

use crate::{tui::{app::{App, FocusedSection, Selection}, operation::{OperationResult, package_expansion::new_scroll_offset_after_expanding}, rendering::{search_bar::render_search_bar, styling::style_border_of_area, tui_layout::{TuiLayout, get_ui_layout, height_of_area}, tui_rows::{TuiRow, render_ui_rows}, visibility::{build_tui_rows, find_index_of_selection_in_visible_rows, to_visible_rows}}}, workspace::types::Package};

pub struct TuiViewportState {
    pub main_section_height: usize,
    pub selected_index: usize,
    pub scroll_offset: usize
}

pub struct TuiView {
    filtered_rows: Vec<TuiRow>
}

impl TuiView {
    pub fn initialize_new(app: &App, search_results: &Vec<Package>) -> TuiView {
        let expanded_set = app.get_current_expanded_packages();
        let rows = build_tui_rows(search_results, expanded_set);

        return TuiView {
            filtered_rows: rows
        };
    }

    pub fn get_filtered_tui_rows(&mut self) -> Vec<TuiRow> {
        return self.filtered_rows.clone();
    }

    fn get_height_from_area(&mut self, area: Rect) -> usize {
        let available_space = height_of_area(area);
        return available_space;
    }

    fn calculate_new_scroll_offset(&mut self, section_height: usize, selected_index: usize, old_scroll_offset: usize) -> usize {
        if section_height == 0 {
            return old_scroll_offset;
        }

        if selected_index < old_scroll_offset {
            return selected_index;
        }

        let viewport_end = old_scroll_offset.saturating_add(section_height);

        if selected_index >= viewport_end {
            return selected_index - section_height + 1;
        }

        return old_scroll_offset;
    }

    fn adjust_scroll_offset(&mut self, calculated_scroll_offset: usize, selected_row: &TuiRow, selected_row_index: usize, main_height: usize, operation_result: &OperationResult) -> usize {
        match operation_result {
            OperationResult::PackageExpanded => {
                return new_scroll_offset_after_expanding(
                    &self.filtered_rows,
                    selected_row,
                    selected_row_index,
                    calculated_scroll_offset,
                    main_height
                );
            },

            _ => {
                return calculated_scroll_offset;
            }
        }
    }

    pub fn get_tui_layout(&mut self, frame: &mut Frame) -> TuiLayout {
        // setup layout
        let terminal_area = frame.area();
        let tui_layout = get_ui_layout(terminal_area);
        return tui_layout;
    }

    pub fn style_tui_layout(&mut self, frame: &mut Frame, mut tui_layout: TuiLayout, focused_section: &FocusedSection) -> TuiLayout {
        let search_focused = matches!(focused_section, FocusedSection::Search);
        let main_focused = matches!(focused_section, FocusedSection::Main);

        // add border to search section
        tui_layout.search = style_border_of_area(frame, tui_layout.search, String::from("Search"), search_focused);

        // add border to main section
        tui_layout.main = style_border_of_area(frame, tui_layout.main, String::from("Scripts"), main_focused);

        return tui_layout;
    }

    pub fn calculate_viewport_state(&mut self, styled_layout: &TuiLayout, current_selection: &Selection, previous_scroll_offset: usize, last_operation_result: &OperationResult) -> TuiViewportState {
        let main_section_height = self.get_height_from_area(styled_layout.main);
        let selected_index_wrapped = find_index_of_selection_in_visible_rows(current_selection, &self.filtered_rows);
        let selected_index = selected_index_wrapped.unwrap_or(0);
        let calculated_scroll_offset = self.calculate_new_scroll_offset(main_section_height, selected_index, previous_scroll_offset);

        let new_scroll_offset = self.adjust_scroll_offset(
            calculated_scroll_offset,
            &current_selection.row,
            selected_index,
            main_section_height,
            last_operation_result
        );

        return TuiViewportState {
            main_section_height: main_section_height,
            selected_index: selected_index,
            scroll_offset: new_scroll_offset
        };
    }

    // TODO: for adding highlighting of where the user is focused, pass focused_section enum to this function and
    // add a match statement in the function (for FocusedSection::Main, highlight border of main section)
    pub fn render(self, frame: &mut Frame, styled_layout: TuiLayout, viewport_state: TuiViewportState, app: &App) -> () {

        // get only the filtered tui rows that are visible (within the area height)
        let visible_rows = to_visible_rows(
            self.filtered_rows,
            viewport_state.scroll_offset,
            viewport_state.main_section_height
        );

        let search_section_focused = app.search_focused();

        render_search_bar(frame, styled_layout.search, &app.app_state.search_input, search_section_focused);
        render_ui_rows(frame, styled_layout.main, app, &visible_rows);
    }

}
