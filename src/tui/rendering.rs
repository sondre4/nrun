// TODO: add truncating script name/command when not enough space
//       add border around center section
//       highlight border (aqua?) of section that is selected
//       add label on top of the column
//       handle expansion of packages in tui
//       handle selection highlighting
//       make navigation up/down and searching
//       add running the selected command
//       add displaying the keybinds available

use ratatui::Frame;

use crate::tui::{app::App, rendering::{search_bar::render_search_bar, styling::add_border_to_area, ui_layout::{UiLayout, get_ui_layout, height_of_area}, ui_rows::{TuiRow, render_ui_rows}, visibility::{build_tui_rows, to_visible_rows}}, scrolling::TuiScrollState};

pub mod search_bar;
pub mod styling;
pub mod ui_keybinds;
pub mod ui_layout;
pub mod ui_rows;
pub mod visibility;

pub struct RenderingData {
    pub visible_rows: Vec<TuiRow>
}

impl RenderingData {
    pub fn new() -> RenderingData {
        return RenderingData {
            visible_rows: Vec::new(),
        }
    }

    pub fn set_visible_rows(mut self, new_visible_rows: Vec<TuiRow>) -> () {
        self.visible_rows = new_visible_rows;
    }
}
pub struct TuiRenderer {
    sections: UiLayout,
    rendering_data: RenderingData,
    scroll_state: TuiScrollState
}

impl TuiRenderer {

    pub fn initialize_layout(&mut self, mut frame: &mut Frame) {
        let layout = get_ui_layout(frame.area());
        self.sections = layout;
        self.sections.main = add_border_to_area(&mut frame, self.sections.main);
    }

    pub fn set_scroll_state(&mut self, top_row_index: usize) -> () {
        let main_space = height_of_area(self.sections.main);

        self.scroll_state = TuiScrollState {
            top_row_index: top_row_index,
            available_space: main_space,
        };
    }

    // runs on every render
    pub fn initialize_rendering_data(mut self, all_tui_rows: Vec<TuiRow>) -> () {
        let visible_rows = to_visible_rows(all_tui_rows, &self.scroll_state);
        self.rendering_data.set_visible_rows(visible_rows);
    }

    // pub fn new(self) -> TuiRenderer {
    //     let sections = self.initialize_layout();

    //     return TuiRenderer {
    //         sections: with_layout,
    //         rendering_data: RenderingData::new()
    //     };
    // }

    // pub fn set_rendering_data(mut self, new_data: RenderingData) -> () {
    //     self.rendering_data = new_data;
    // }

    pub fn get_rendering_data(self) -> RenderingData {
        return self.rendering_data;
    }

    pub fn render(self, mut frame: Frame, app: &App) -> () {
        render_search_bar(&mut frame, self.sections.search, app);
        render_ui_rows(&mut frame, self.sections.main, app, &self.rendering_data.visible_rows);
    }

}

// pub fn render(frame: &mut Frame, app: &App) -> () {
//     // let layout = get_ui_layout(frame.area());

//     // let main_section = get_area_with_border(frame, layout.main);

//     render_search_bar(frame, layout.search, app);
//     render_ui_rows(frame, main_section, app, &app.rendering_data.visible_rows);
// }
