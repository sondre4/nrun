use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, widgets::{Block, Borders, Paragraph}};

use crate::tui::app::App;

struct UiLayout {
    search: Rect,
    list: Rect,
    info: Rect,
    keybind_display: Rect
}

fn create_layout(area: Rect) -> UiLayout {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1)
        ]).split(area);

    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60)
        ]).split(vertical[1]);

    let layout = UiLayout {
        search: vertical[0],
        list: main[0],
        info: main[1],
        keybind_display: vertical[1]
    };

    return layout;
}

fn render_search_bar(frame: &mut Frame, app: &App, area: Rect) -> () {
    let search_widget = Paragraph::new(app.search_query.to_string()).block(
        Block::default().borders(Borders::ALL).title(" Search ")
    );

    frame.render_widget(search_widget, area);
}

pub fn render(frame: &mut Frame, app: &App) -> () {
    let layout = create_layout(frame.area());

    render_search_bar(frame, app, layout.search);
}
