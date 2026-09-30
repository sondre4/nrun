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

use crate::tui::{app::{App, FocusedSection, Selection}, rendering::{search_bar::render_search_bar, styling::add_border_to_area, tui_layout::{TuiLayout, get_ui_layout, height_of_area}, tui_rows::{TuiRow, render_ui_rows}, visibility::{build_tui_rows, find_index_of_selection_in_visible_rows, to_visible_rows}}, searching::filter_tui_rows};

pub mod search_bar;
pub mod styling;
pub mod tui_keybinds;
pub mod tui_layout;
pub mod tui_rows;
pub mod tui_view;
pub mod visibility;





