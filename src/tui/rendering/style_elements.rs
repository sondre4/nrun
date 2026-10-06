use ratatui::{Frame, layout::{Margin, Rect}, style::{Color, Modifier, Style}, widgets::{Block, Borders}};

use crate::tui::{app::Selection, rendering::tui_rows::TuiRow, styles::{colors::ColorPalette, ui_style::{KeybindDisplayUiStyle, PackageUiStyle, ScriptUiStyle, UiStyle, UiStyles, VSpaceUiStyle}}};

fn get_row_style(visible_row: &TuiRow, selection: &Selection, styles: &UiStyles) -> UiStyle {
    // 5 cases:
    // script name is selected => both selection.package_id and selection.script_id
    // package name is selected => only selection.package_id
    // script unselected
    // package unselected
    // package relevant

    let selected_row = &selection.row;
    let visible_row_data_option = visible_row.get_row_data();
    let selected_row_data_option = selected_row.get_row_data();

    match visible_row {
        TuiRow::Package { package_id } => {
            let selected_row = selected_row_data_option.unwrap();

            // println!("Visible row is a package");

            // case 1: relevant package - both package_id and script_id selected
            if *package_id == selected_row.package_id && selected_row.script_id.is_some() {
                return styles.relevant_package;
            }

            // case 2: selected package - only package_id, because no script is selected
            if *package_id == selected_row.package_id && selected_row.script_id.is_none() {
                return styles.selected_package;
            }

            // case 3: package is not selected, package_id does not match visible_row package_id
            return styles.unselected_package;
        },

        TuiRow::Script { package_id, script_id } => {
            let selected_row_data = selected_row_data_option.unwrap();

            if visible_row_data_option.is_none() {
                unreachable!("Unable to get display style for script: No script id specified");
            }

            if selected_row_data.script_id.is_none() {
                // for some reason this fixes the bug
                // also case 5?: script is not selected
                return styles.unselected_script;
            }

            // case 4: script is selected, identified by both script_id and package_id
            if *package_id == selected_row_data.package_id && *script_id == selected_row_data.script_id.unwrap() {
                return styles.selected_script;
            }

            // case 5: script is not selected
            return styles.unselected_script;
        },

        TuiRow::Vspace => {
            return styles.vspace_style;
        }
    }
}


pub fn get_package_row_style(visible_row: &TuiRow, selection: &Selection, styles: &UiStyles) -> PackageUiStyle {
    let style = get_row_style(visible_row, selection, styles);
    match style {
        UiStyle::PackageStyle(package_ui_style) => {
            return package_ui_style;
        },
        _ => unreachable!("Row style should not be other than style for package row")
    };
}

pub fn get_script_row_style(visible_row: &TuiRow, selection: &Selection, styles: &UiStyles) -> ScriptUiStyle {
    let style = get_row_style(visible_row, selection, styles);
    match style {
        UiStyle::ScriptStyle(script_ui_style) => {
            return script_ui_style;
        },
        _ => unreachable!("Row style should not be other than style for script row")
    };
}

pub fn get_vspace_row_style(visible_row: &TuiRow, selection: &Selection, styles: &UiStyles) -> VSpaceUiStyle {
    let style = get_row_style(visible_row, selection, styles);
    match style {
        UiStyle::VspaceStyle(vspace_ui_style) => {
            return vspace_ui_style;
        },
        _ => unreachable!("Row style should not be other than style for vspace row")
    };
}

pub fn get_keybind_display_style(styles: &UiStyles) -> KeybindDisplayUiStyle {
    let style = styles.keybind_display;
    match style {
        UiStyle::KeybindDisplayStyle(keybind_display_style) => {
            return keybind_display_style;
        },
        _ => unreachable!("Keybind display style should not be other than style for keybind display")
    };
}

pub fn add_margin_to_area(frame: &mut Frame, area: Rect, margin_x: u16, margin_y: u16) -> Rect {
    let inner = area.inner(Margin {
        horizontal: margin_x,
        vertical: margin_y
    });

    return inner;
}

pub fn style_border_of_area(frame: &mut Frame, area: Rect, title: String, focused: bool, color_palette: &ColorPalette) -> Rect {
    let style: Style = if focused {
        Style::new().fg(color_palette.primary)
    } else {
        Style::new().fg(color_palette.border)
    };

    // add border on area
    let border = Block::default().borders(Borders::ALL).border_style(style).title(format!(" {title} "));
    frame.render_widget(border, area);

    // let inner_area = area.inner(Margin { vertical: 1, horizontal: 1 });
    let inner_area = add_margin_to_area(frame, area, 1, 1);
    return inner_area;
}
