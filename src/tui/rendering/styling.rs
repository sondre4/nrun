use ratatui::{Frame, layout::{Margin, Rect}, style::{Color, Modifier, Style}, widgets::{Block, Borders}};

const PACKAGE_NAME_STYLE: Style = Style::new()
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD);

const SCRIPT_NAME_STYLE: Style = Style::new()
    .fg(Color::White);

const SCRIPT_COMMAND_STYLE: Style = Style::new()
    .fg(Color::DarkGray);

const SELECTED_STYLE: Style = Style::new()
    .fg(Color::Black)
    .bg(Color::Cyan);



// fn get_package_name_style() -> Style {

// }

// fn get_script_name_style() -> Style {

// }

// fn get_script_command_style() -> Style {

// }

// fn get_selected_style()

// fn get_unselected_style()

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
