// TODO: add truncating script name/command when not enough space
//       add border around center section
//       highlight border (aqua?) of section that is selected
//       add label on top of the column
//       handle expansion of packages in tui
//       handle selection highlighting
//       make navigation up/down and searching
//       add running the selected command
//       add displaying the keybinds available

use std::io;

use ratatui::{Frame, Terminal, backend::CrosstermBackend, layout::Rect};

use crate::tui::{app::{App, FocusedSection, Selection}, rendering::{search_bar::render_search_bar, styling::add_border_to_area, ui_layout::{TuiLayout, get_ui_layout, height_of_area}, ui_rows::{TuiRow, render_ui_rows}, visibility::{build_tui_rows, find_index_of_selection_in_visible_rows, to_visible_rows}}, searching::filter_tui_rows};

pub mod search_bar;
pub mod styling;
pub mod ui_keybinds;
pub mod ui_layout;
pub mod ui_rows;
pub mod visibility;

pub struct TuiViewportState {
    pub main_section_height: usize,
    pub selected_index: usize,
    pub scroll_offset: usize
}

pub struct TuiView {
    filtered_rows: Vec<TuiRow>
}

impl TuiView {
    pub fn initialize_new(from_app: &App) -> TuiView {
        let all_rows = build_tui_rows(from_app);
        let filtered_rows = filter_tui_rows(all_rows, &from_app.app_state.search_query);

        return TuiView {
            filtered_rows: filtered_rows
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

    pub fn get_tui_layout(&mut self, frame: &mut Frame) -> TuiLayout {
        // setup layout
        let terminal_area = frame.area();
        let tui_layout = get_ui_layout(terminal_area);
        return tui_layout;
    }

    pub fn style_tui_layout(&mut self, frame: &mut Frame, mut tui_layout: TuiLayout, focused_section: &FocusedSection) -> TuiLayout {
        // add border to main section
        tui_layout.main = add_border_to_area(frame, tui_layout.main);
        return tui_layout;
    }

    pub fn calculate_viewport_state(&mut self, styled_layout: &TuiLayout, current_selection: &Selection, previous_scroll_offset: usize) -> TuiViewportState {
        let main_section_height = self.get_height_from_area(styled_layout.main);
        let selected_index_wrapped = find_index_of_selection_in_visible_rows(current_selection, &self.filtered_rows);
        let selected_index = selected_index_wrapped.unwrap_or(0);
        let new_scroll_offset = self.calculate_new_scroll_offset(main_section_height, selected_index, previous_scroll_offset);

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

        render_search_bar(frame, styled_layout.search, app);
        render_ui_rows(frame, styled_layout.main, app, &visible_rows);
    }

}

// pub fn render(frame: &mut Frame, app: &App) -> () {
//     // let layout = get_ui_layout(frame.area());

//     // let main_section = get_area_with_border(frame, layout.main);

//     render_search_bar(frame, layout.search, app);
//     render_ui_rows(frame, main_section, app, &app.rendering_data.visible_rows);
// }
