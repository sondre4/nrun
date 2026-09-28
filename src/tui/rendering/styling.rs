use ratatui::{Frame, layout::{Margin, Rect}, style::{Color, Modifier, Style}, widgets::{Block, Borders}};

struct UiStyle {
    package_name: Style,
    script_name: Style,
    script_command: Style,

    // when a package's script is selected
    relevant_package_name: Style
}

struct UiStyles {
    normal: UiStyle,
    selected: UiStyle
}

impl Default for UiStyles {
    fn default() -> Self {

        let normal_style: UiStyle = UiStyle {
            package_name: Style::default(),
            script_name: Style::default(),
            script_command: Style::default()
                .fg(Color::DarkGray),

            relevant_package_name: Style::default(),
        };

        let selected_style: UiStyle = UiStyle {
            package_name: Style::default()
                .fg(Color::Blue)
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD),

            script_name: Style::default()
                .fg(Color::Blue)
                .bg(Color::White),
            script_command: Style::default()
                .fg(Color::Gray)
                .bg(Color::White),
            relevant_package_name: Style::default()
                .fg(Color::DarkGray)
                .bg(Color::Gray)
        };


        let ui_styles: UiStyles = UiStyles {
            normal: normal_style,
            selected: selected_style,
        };

        return ui_styles;
    }
}


// fn get_package_name_style() -> Style {

// }

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
