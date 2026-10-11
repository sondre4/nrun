use std::{cmp::max, collections::HashMap};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::{
    action::Action,
    app::FocusedSection,
    keybinds::{Keybind, MAIN_KEYBINDS, SEARCH_KEYBINDS, get_keybind_name},
    rendering::style_elements::get_keybind_display_style,
    styles::ui_style::KeybindDisplayUiStyle,
};

struct DisplayedKeybindEntry {
    action: Action,
    description: String,
    keys: Vec<String>,
}

fn choose_keybind_set(focused_section: &FocusedSection) -> Vec<&'static Keybind> {
    let mut keybinds: Vec<&Keybind> = Vec::new();

    match focused_section {
        FocusedSection::Main => {
            for keybind in MAIN_KEYBINDS {
                keybinds.push(keybind);
            }
        }

        FocusedSection::Search => {
            for keybind in SEARCH_KEYBINDS {
                keybinds.push(keybind);
            }
        }
    }

    return keybinds;
}

fn keybinds_to_display_entries(keybinds: Vec<&Keybind>) -> Vec<DisplayedKeybindEntry> {
    let mut entries: Vec<DisplayedKeybindEntry> = Vec::new();
    let mut completed_keybind_actions: Vec<Action> = Vec::new();
    let keybind_count = keybinds.len();

    // Squared time complexity is ok here because there are not many keybinds
    for outer_index in 0..keybind_count {
        let keybind = &keybinds[outer_index];
        let current_action = keybind.action;

        if completed_keybind_actions.contains(&current_action) {
            continue;
        }

        let mut keybind_names: Vec<String> = Vec::new();

        for inner_index in 0..keybind_count {
            let inner_keybind = &keybinds[inner_index];

            if inner_keybind.action == current_action {
                let key_name = get_keybind_name(inner_keybind);
                keybind_names.push(key_name);
            }
        }

        completed_keybind_actions.push(current_action);

        let keybind_entry: DisplayedKeybindEntry = DisplayedKeybindEntry {
            action: current_action,
            description: keybind.description.to_string(),
            keys: keybind_names,
        };

        entries.push(keybind_entry);
    }

    return entries;
}

fn build_keybind_map(keybinds: Vec<&Keybind>) -> HashMap<Action, Vec<Keybind>> {
    let mut keybind_map: HashMap<Action, Vec<Keybind>> = HashMap::new();

    for keybind in keybinds {
        let action = keybind.action;

        let keybind_collection = keybind_map.entry(action).or_default();
        keybind_collection.push(keybind.clone());
    }

    return keybind_map;
}

fn build_keybind_display_data(keybind_map: &HashMap<Action, Vec<Keybind>>) -> Vec<DisplayedKeybindEntry> {
    let mut entries: Vec<DisplayedKeybindEntry> = Vec::new();

    for (action, keybinds) in keybind_map {
        let main_keybind_description: String = String::from(keybinds[0].description);
        let mut keybind_description: String = main_keybind_description.clone();
        let mut keybind_keys: Vec<String> = Vec::new();

        for keybind in keybinds {
            let key_name = get_keybind_name(keybind);
            keybind_keys.push(key_name);

            let description = String::from(keybind.description);

            if description != main_keybind_description {
                keybind_description.push_str(format!("/{}", description).as_str());
            }
        }

        let entry: DisplayedKeybindEntry = DisplayedKeybindEntry {
            action: *action,
            description: keybind_description,
            keys: keybind_keys,
        };

        entries.push(entry);
    }

    return entries;
}

fn get_padding_spans(top_column_count: usize, bottom_column_count: usize, pad_length: usize) -> Vec<Span<'static>> {
    let end_length = max(top_column_count, bottom_column_count) + pad_length;

    let top_extra_length = end_length.saturating_sub(top_column_count);
    let bottom_extra_length = end_length.saturating_sub(bottom_column_count);

    let top_extra_span = Span::from(" ".repeat(top_extra_length));
    let bottom_extra_span = Span::from(" ".repeat(bottom_extra_length));

    let mut padding_spans: Vec<Span> = Vec::new();
    padding_spans.push(top_extra_span);
    padding_spans.push(bottom_extra_span);

    return padding_spans;
}

fn build_display_lines(display_entries: Vec<DisplayedKeybindEntry>, style: &KeybindDisplayUiStyle) -> Vec<Line<'static>> {
    let mut display_lines: Vec<Line> = Vec::new();
    let mut top_line: Line = Line::default();
    let mut bottom_line: Line = Line::default();

    let keycode_separator = " / ";

    for entry in display_entries {
        let mut new_colums_top_count: usize = 0;
        let mut new_colums_bottom_count: usize = 0;
        let padding_length: usize = 3;

        for i in 0..entry.keys.len() {
            let keycode_name = &entry.keys[i];

            if i != 0 {
                // update count of how many columns have been added
                new_colums_top_count = new_colums_top_count.saturating_add(keycode_separator.chars().count());
                let keycode_separator_span = Span::from(keycode_separator).style(style.separator);
                top_line.push_span(keycode_separator_span);
            }

            // update count of how many columns have been added
            new_colums_top_count = new_colums_top_count.saturating_add(keycode_name.chars().count());
            let keycode_separator_span = Span::from(keycode_name.clone()).style(style.keybinds);
            top_line.push_span(keycode_separator_span);
        }

        // update count of how many columns have been added
        new_colums_bottom_count = new_colums_bottom_count.saturating_add(entry.description.chars().count());
        let description_span = Span::from(entry.description).style(style.description);
        bottom_line.push_span(description_span);

        let padding_spans = get_padding_spans(new_colums_top_count, new_colums_bottom_count, padding_length);

        top_line.push_span(padding_spans[0].clone());
        bottom_line.push_span(padding_spans[1].clone());
    }

    display_lines.push(top_line);
    display_lines.push(bottom_line);

    return display_lines;
}

pub fn render_tui_keybinds(frame: &mut Frame, area: Rect, focused_section: &FocusedSection, style: &KeybindDisplayUiStyle) -> () {
    let keybinds = choose_keybind_set(focused_section);
    let keybind_display_data = keybinds_to_display_entries(keybinds);
    let display_lines = build_display_lines(keybind_display_data, style);

    let [keybind_name_row, description_row] = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);

    let display_name_row = Paragraph::new(display_lines[0].clone());
    let display_description = Paragraph::new(display_lines[1].clone());

    frame.render_widget(display_name_row, keybind_name_row);
    frame.render_widget(display_description, description_row);
}
