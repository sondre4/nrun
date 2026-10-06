use ratatui::style::{Color, Modifier, Style};

use crate::tui::styles::colors::ColorPalette;

#[derive(Clone, Copy)]
pub struct PackageUiStyle {
    pub name: Style
}

#[derive(Clone, Copy)]
pub struct ScriptUiStyle {
    pub name: Style,
    pub command: Style
}

#[derive(Clone, Copy)]
pub struct VSpaceUiStyle {
    pub style: Style
}

#[derive(Clone, Copy)]
pub struct KeybindDisplayUiStyle {
    pub keybinds: Style,
    pub separator: Style,
    pub description: Style
}

#[derive(Clone, Copy)]
pub enum UiStyle {
    PackageStyle(PackageUiStyle),
    ScriptStyle(ScriptUiStyle),
    VspaceStyle(VSpaceUiStyle),
    KeybindDisplayStyle(KeybindDisplayUiStyle)
}

#[derive(Clone, Copy)]
pub struct UiStyles {
    pub selected_package: UiStyle,
    pub relevant_package: UiStyle,
    pub unselected_package: UiStyle,
    pub selected_script: UiStyle,
    pub unselected_script: UiStyle,
    pub vspace_style: UiStyle,
    pub keybind_display: UiStyle
}

impl UiStyles {
    pub fn from_palette(palette: ColorPalette) -> Self {

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
