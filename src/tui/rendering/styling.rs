use ratatui::{Frame, layout::{Margin, Rect}, style::Style, widgets::{Block, Borders}};

struct HighlightedStyles {
    name_column: Style,
    command_column: Style
}



// fn is_selected()

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
