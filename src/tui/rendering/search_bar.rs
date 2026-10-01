use ratatui::{Frame, layout::Rect, widgets::{Block, Borders, Paragraph}};

use crate::tui::app::App;

pub fn render_search_bar(frame: &mut Frame, area: Rect, app: &App) -> () { // TODO: add style parameter
    let search_widget = Paragraph::new(app.app_state.search_input.value()).block(
        Block::default().borders(Borders::ALL).title(" Search ")
    );

    frame.render_widget(search_widget, area);
}
