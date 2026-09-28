use ratatui::{Frame, layout::{Margin, Rect}, style::{Color, Modifier, Style}, widgets::{Block, Borders}};

use crate::{tui::app::Selection, workspace::types::Package};

pub struct UiStyle {
    pub package_name: Style,
    pub script_name: Style,
    pub script_command: Style,
}

pub struct UiStyles {
    pub script_name_selected: UiStyle,
    pub package_name_selected: UiStyle
}

impl Default for UiStyles {
    fn default() -> Self {

        let script_selected: UiStyle = UiStyle {
            package_name: Style::default()
                .fg(Color::DarkGray)
                .bg(Color::Gray),

            script_name: Style::default()
                .fg(Color::Blue)
                .bg(Color::White),

            script_command: Style::default()
                .fg(Color::Gray)
                .bg(Color::White)
        };

        let package_selected: UiStyle = UiStyle {
            package_name: Style::default()
                .fg(Color::Blue)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),

            script_name: Style::default(),

            script_command: Style::default()
                .fg(Color::DarkGray)
        };

        let ui_styles: UiStyles = UiStyles {
            script_name_selected: script_selected,
            package_name_selected: package_selected
        };

        return ui_styles;
    }
}


pub fn get_row_style(package: &Package, selection: &Selection) -> UiStyle {
    // 2 cases:
    // script name is selected => both selection.package_id and selection.script_id
    // package name is selected => only selection.package_id

    let ui_styles = UiStyles::default();

    if selection.script_id.is_some() {
        // case 1: script id is selected
        return ui_styles.script_name_selected;
    }

    // case 2: package name is selected
    return ui_styles.package_name_selected;
}

// style
// fn get_list_style(package: &Package, selection: &Selection) -> Style {
//     // 2 cases:
//     // script name is selected => both selection.package_id and selection.script_id
//     // package name is selected => only selection.package_id

//     if selection.script_id.is_some() {
//         // case 1: package id is selected




//     } else {
//         // case 2: package name is selected
//     }
// }

pub fn get_area_with_border(frame: &mut Frame, area: Rect) -> Rect {
    // add border on area
    let border = Block::default().borders(Borders::ALL).title(" Scripts ");
    frame.render_widget(border, area);

    let inner_area = area.inner(Margin { vertical: 1, horizontal: 1 });
    return inner_area;
}
