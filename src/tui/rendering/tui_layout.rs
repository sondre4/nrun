use ratatui::layout::{Constraint, Direction, Layout, Rect};

#[derive(Clone)]
pub struct TuiLayout {
    pub search: Rect,
    pub main: Rect,
    pub keybind_display: Rect,
}

fn create_layout(area: Rect) -> TuiLayout {
    let sections =
        Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(4)]).split(area);

    let layout = TuiLayout {
        search: sections[0],
        main: sections[1],
        keybind_display: sections[2],
    };

    return layout;
}

pub fn get_ui_layout(area: Rect) -> TuiLayout {
    return create_layout(area);
}

pub fn height_of_area(area: Rect) -> usize {
    let height = area.height;
    return usize::from(height);
}
