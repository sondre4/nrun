use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct UiLayout {
    pub search: Rect,
    pub main: Rect,
    pub keybind_display: Rect
}

fn create_layout(area: Rect) -> UiLayout {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1)
        ]).split(area);

    // let main = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([
    //         Constraint::Percentage(40),
    //         Constraint::Percentage(60)
    //     ]).split(vertical[1]);

    let layout = UiLayout {
        search: sections[0],
        main: sections[1],
        keybind_display: sections[2]
    };

    return layout;
}

pub fn get_ui_layout(area: Rect) -> UiLayout {
    return create_layout(area);
}
