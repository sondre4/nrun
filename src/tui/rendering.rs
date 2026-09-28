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

use crate::tui::{app::App, rendering::{search_bar::render_search_bar, styling::get_area_with_border, ui_layout::get_ui_layout, ui_rows::render_ui_rows}};

pub mod search_bar;
pub mod styling;
pub mod ui_keybinds;
pub mod ui_layout;
pub mod ui_rows;
pub mod visibility;


pub fn render(frame: &mut Frame, app: &App) -> () {
    let layout = get_ui_layout(frame.area());

    let main_section = get_area_with_border(frame, layout.main);

    render_search_bar(frame, layout.search, app);
    render_ui_rows(frame, main_section, app);
}
