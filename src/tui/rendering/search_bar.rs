use ratatui::{
    Frame,
    layout::{Position, Rect},
    widgets::Paragraph,
};
use tui_input::Input;

pub fn render_search_bar(frame: &mut Frame, search_bar_section: Rect, input: &Input, mut render_cursor: bool) -> () {
    // TODO: add style parameter
    let search_widget = Paragraph::new(input.value());

    let section_x = search_bar_section.x;
    let section_y = search_bar_section.y;
    let cursor_x_display_columns = u16::try_from(input.visual_cursor());

    let mut cursor_x_columns: u16 = 0;

    match cursor_x_display_columns {
        Ok(result) => {
            cursor_x_columns = result;
        }
        Err(_error) => {
            // value larger than u16 limit
            cursor_x_columns = 0;
            render_cursor = false;
        }
    }

    if render_cursor {
        let cursor_x = section_x.saturating_add(cursor_x_columns);
        let cursor_coordinates: Position = Position::new(cursor_x, section_y);
        frame.set_cursor_position(cursor_coordinates);
    }

    frame.render_widget(search_widget, search_bar_section);
}
