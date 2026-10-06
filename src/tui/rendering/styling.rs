use ratatui::{Frame, layout::{Margin, Rect}, style::{Color, Modifier, Style}, widgets::{Block, Borders}};

use crate::tui::{app::Selection, rendering::tui_rows::TuiRow};

pub struct PackageUiStyle {
    pub name: Style
}

pub struct ScriptUiStyle {
    pub name: Style,
    pub command: Style
}

pub struct VSpaceUiStyle {
    pub style: Style
}

pub struct KeybindDisplayUiStyle {
    pub keybinds: Style,
    pub separator: Style,
    pub description: Style
}

pub enum UiStyle {
    PackageStyle(PackageUiStyle),
    ScriptStyle(ScriptUiStyle),
    VspaceStyle(VSpaceUiStyle),
    KeybindDisplayStyle(KeybindDisplayUiStyle)
}

pub struct UiStyles {
    // pub script_name: UiStyle,
    // pub package_name: UiStyle,
    // pub script_name_selected: UiStyle,
    // pub package_name_selected: UiStyle

    selected_package: UiStyle,
    relevant_package: UiStyle,
    unselected_package: UiStyle,
    selected_script: UiStyle,
    unselected_script: UiStyle,
    vspace_style: UiStyle,
    keybind_display: UiStyle
}

impl Default for UiStyles {
    fn default() -> Self {

        // TODO: once focused_section is set, set variables to be different colors depending on FocusedSection enum
        // could be implemented with a focused bool?

        // package styles
        let unselected_package_style: PackageUiStyle = PackageUiStyle {
            name: Style::default(),
        };

        let relevant_package_style: PackageUiStyle = PackageUiStyle {
            name: Style::default()
                .fg(Color::LightCyan)
                .bg(Color::DarkGray)
        };

        let selected_package_style: PackageUiStyle = PackageUiStyle {
            name: Style::default()
                .fg(Color::Blue)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        };

        // script styles
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

        let vspace_style: VSpaceUiStyle = VSpaceUiStyle {
            style: Style::default().fg(Color::Black).bg(Color::Black)
        };

        let keybind_display_style: KeybindDisplayUiStyle = KeybindDisplayUiStyle {
            keybinds: Style::default().fg(Color::Cyan),
            separator: Style::default().fg(Color::DarkGray),
            description: Style::default().fg(Color::LightCyan)
        };

        let ui_styles: UiStyles = UiStyles {
            selected_package: UiStyle::PackageStyle(selected_package_style),
            relevant_package: UiStyle::PackageStyle(relevant_package_style),
            unselected_package: UiStyle::PackageStyle(unselected_package_style),
            selected_script: UiStyle::ScriptStyle(script_selected),
            unselected_script: UiStyle::ScriptStyle(script_unselected),
            vspace_style: UiStyle::VspaceStyle(vspace_style),
            keybind_display: UiStyle::KeybindDisplayStyle(keybind_display_style)
        };

        return ui_styles;
    }
}

fn get_row_style(visible_row: &TuiRow, selection: &Selection) -> UiStyle {
    // 5 cases:
    // script name is selected => both selection.package_id and selection.script_id
    // package name is selected => only selection.package_id
    // script unselected
    // package unselected
    // package relevant

    // println!("getting row style for {visible_row:#?}");

    let selected_row = &selection.row;

    let ui_styles = UiStyles::default();
    // let visible_row_data_option = visible_row.get_row_data();
    
    let visible_row_data_option = visible_row.get_row_data();
    let selected_row_data_option = selected_row.get_row_data();

    match visible_row {
        TuiRow::Package { package_id } => {
            let selected_row = selected_row_data_option.unwrap();

            // println!("Visible row is a package");

            // case 1: relevant package - both package_id and script_id selected
            if *package_id == selected_row.package_id && selected_row.script_id.is_some() {
                return ui_styles.relevant_package;
            }

            // case 2: selected package - only package_id, because no script is selected
            if *package_id == selected_row.package_id && selected_row.script_id.is_none() {
                return ui_styles.selected_package;
            }

            // case 3: package is not selected, package_id does not match visible_row package_id
            return ui_styles.unselected_package;
        },

        TuiRow::Script { package_id, script_id } => {
            let selected_row_data = selected_row_data_option.unwrap();

            if visible_row_data_option.is_none() {
                unreachable!("Unable to get display style for script: No script id specified");
            }

            if selected_row_data.script_id.is_none() {
                // for some reason this fixes the bug
                // also case 5?: script is not selected
                return ui_styles.unselected_script;
            }

            // case 4: script is selected, identified by both script_id and package_id
            if *package_id == selected_row_data.package_id && *script_id == selected_row_data.script_id.unwrap() {
                return ui_styles.selected_script;
            }

            // case 5: script is not selected
            return ui_styles.unselected_script;
        },

        TuiRow::Vspace => {
            return ui_styles.vspace_style;
        }
    }
}

pub fn get_package_row_style(visible_row: &TuiRow, selection: &Selection) -> PackageUiStyle {
    let style = get_row_style(visible_row, selection);
    match style {
        UiStyle::PackageStyle(package_ui_style) => {
            return package_ui_style;
        },
        _ => unreachable!("Row style should not be other than style for package row")
    };
}

pub fn get_script_row_style(visible_row: &TuiRow, selection: &Selection) -> ScriptUiStyle {
    let style = get_row_style(visible_row, selection);
    match style {
        UiStyle::ScriptStyle(script_ui_style) => {
            return script_ui_style;
        },
        _ => unreachable!("Row style should not be other than style for script row")
    };
}

pub fn get_vspace_row_style(visible_row: &TuiRow, selection: &Selection) -> VSpaceUiStyle {
    let style = get_row_style(visible_row, selection);
    match style {
        UiStyle::VspaceStyle(vspace_ui_style) => {
            return vspace_ui_style;
        },
        _ => unreachable!("Row style should not be other than style for vspace row")
    };
}

pub fn get_keybind_display_style() -> KeybindDisplayUiStyle {
    let style = UiStyles::default().keybind_display;
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

pub fn style_border_of_area(frame: &mut Frame, area: Rect, title: String, focused: bool) -> Rect {
    let style: Style = if focused {
        Style::new().fg(Color::Cyan)
    } else {
        Style::new().fg(Color::LightCyan)
    };

    // add border on area
    let border = Block::default().borders(Borders::ALL).border_style(style).title(format!(" {title} "));
    frame.render_widget(border, area);

    // let inner_area = area.inner(Margin { vertical: 1, horizontal: 1 });
    let inner_area = add_margin_to_area(frame, area, 1, 1);
    return inner_area;
}
