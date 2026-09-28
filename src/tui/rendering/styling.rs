use ratatui::{Frame, layout::{Margin, Rect}, style::{Color, Modifier, Style}, widgets::{Block, Borders}};

use crate::{tui::app::Selection, workspace::types::{Package, Script}};

// pub struct UiStyle {
//     pub package_name: Style,
//     pub script_name: Style,
//     pub script_command: Style,
// }

pub struct PackageUiStyle {
    pub name: Style
}

pub struct ScriptUiStyle {
    pub name: Style,
    pub command: Style
}

pub struct UiStyles {
    // pub script_name: UiStyle,
    // pub package_name: UiStyle,
    // pub script_name_selected: UiStyle,
    // pub package_name_selected: UiStyle

    selected_package: PackageUiStyle,
    relevant_package: PackageUiStyle,
    unselected_package: PackageUiStyle,
    selected_script: ScriptUiStyle,
    unselected_script: ScriptUiStyle
}

impl Default for UiStyles {
    fn default() -> Self {

        let unselected_package_style: PackageUiStyle = PackageUiStyle {
            name: Style::default(),
        };

        let relevant_package_style: PackageUiStyle = PackageUiStyle {
            name: Style::default()
                .fg(Color::DarkGray)
                .bg(Color::Gray)
        };

        let selected_package_style: PackageUiStyle = PackageUiStyle {
            name: Style::default()
                .fg(Color::Blue)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        };



        let script_unselected: ScriptUiStyle = ScriptUiStyle {
            name: Style::default(),
            command: Style::default().fg(Color::DarkGray)
        };

        let script_selected: ScriptUiStyle = ScriptUiStyle {
            name: Style::default()
                .fg(Color::Blue)
                .bg(Color::White),

            command: Style::default()
                .fg(Color::DarkGray)
                .bg(Color::White)
        };

        let ui_styles: UiStyles = UiStyles {
            selected_package: selected_package_style,
            relevant_package: relevant_package_style,
            unselected_package: unselected_package_style,
            selected_script: script_selected,
            unselected_script: script_unselected
        };

        return ui_styles;
    }
}

pub fn get_package_row_style(package: &Package, selection: &Selection) -> PackageUiStyle {
    // 2 cases:
    // script name is selected => both selection.package_id and selection.script_id
    // package name is selected => only selection.package_id

    let ui_styles = UiStyles::default();

    // check if package is part of selection in the first place
    if package.id == selection.package_id {

        // check if a script is also selected
        // if a script is selected here,
        // then one of this package's scripts is part of the selection

        if selection.script_id.is_some() {
            // one of this package's scripts are selected
            return ui_styles.relevant_package;
        }

        // if no script is part of the selection, then a package name is selected
        return ui_styles.selected_package;
    }

    // if not, return unselected style
    return ui_styles.unselected_package;
}

pub fn get_script_row_style(script: &Script, selection: &Selection) -> ScriptUiStyle {
    // 2 cases:
    // script name is selected => both selection.package_id and selection.script_id
    // package name is selected => only selection.package_id

    let ui_styles = UiStyles::default();

    if selection.script_id.is_none() {
        return ui_styles.unselected_script;
    }

    if script.id == selection.script_id.unwrap() {
        return ui_styles.selected_script;
    }

    return ui_styles.unselected_script
}

pub fn get_area_with_border(frame: &mut Frame, area: Rect) -> Rect {
    // add border on area
    let border = Block::default().borders(Borders::ALL).title(" Scripts ");
    frame.render_widget(border, area);

    let inner_area = area.inner(Margin { vertical: 1, horizontal: 1 });
    return inner_area;
}
