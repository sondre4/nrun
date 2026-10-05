use std::{cmp::max, collections::HashMap};

use ratatui::{Frame, layout::{Constraint, Layout, Rect}, text::{Line, Span}, widgets::Paragraph};

use crate::tui::{action::Action, app::FocusedSection, keybinds::{Keybind, MAIN_KEYBINDS, SEARCH_KEYBINDS, get_keybind_name}, rendering::styling::get_keybind_display_style};

struct DisplayedKeybindEntry {
    action: Action,
    description: String,
    keys: Vec<String>
}

fn choose_keybind_set(focused_section: &FocusedSection) -> Vec<&'static Keybind> {
    let mut keybinds: Vec<&Keybind> = Vec::new();

    match focused_section {
        FocusedSection::Main => {
            for keybind in MAIN_KEYBINDS {
                keybinds.push(keybind);
            }
        },

        FocusedSection::Search => {
            for keybind in SEARCH_KEYBINDS {
                keybinds.push(keybind);
            }
        }
    }

    return keybinds;
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
            keys: keybind_keys
        };

        entries.push(entry);
    }

    return entries;
}

fn align_and_pad_strings(mut s1: String, mut s2: String, pad_length: usize) -> Vec<String> {
    let max_length = max(s1.chars().count(), s2.chars().count());

    while s1.chars().count() < max_length {
        s1.push_str(" ");
    }

    while s2.chars().count() < max_length {
        s2.push_str(" ");
    }

    s1.push_str(&" ".repeat(pad_length));
    s2.push_str(&" ".repeat(pad_length));

    let mut aligned_strings: Vec<String> = Vec::new();
    aligned_strings.push(s1);
    aligned_strings.push(s2);

    return aligned_strings;
}

fn build_display_lines(display_entries: Vec<DisplayedKeybindEntry>) -> Vec<Line<'static>> {
    let mut display_lines: Vec<Line> = Vec::new();
    let mut top_line: Line = Line::default();
    let mut bottom_line: Line = Line::default();

    for entry in display_entries {
        let mut keybind_keys = String::new();

        for i in 0..entry.keys.len() {
            let key = &entry.keys[i];

            if i != 0 {
                keybind_keys.push_str(" / ");
            }

            keybind_keys.push_str(key);
        }

        let aligned_strings = align_and_pad_strings(keybind_keys, entry.description, 3);

        let top_span = Span::from(aligned_strings[0].clone());
        let bottom_span = Span::from(aligned_strings[1].clone());

        top_line.push_span(top_span);
        bottom_line.push_span(bottom_span);
    }

    display_lines.push(top_line);
    display_lines.push(bottom_line);

    return display_lines;
}

pub fn render_tui_keybinds(frame: &mut Frame, area: Rect, focused_section: &FocusedSection) -> () {
    let keybinds = choose_keybind_set(focused_section);
    let keybind_map = build_keybind_map(keybinds);
    let keybind_display_data = build_keybind_display_data(&keybind_map);
    let display_lines = build_display_lines(keybind_display_data);

    let style = get_keybind_display_style();

    let [keybind_name_row, description_row] = Layout::vertical(
        [
            Constraint::Length(1),
            Constraint::Length(1)
        ]
    ).areas(area);

    // let mut styled_display_lines: Vec<Line> = Vec::new();
    // styled_display_lines.push(display_lines[0].clone().style(style.keybinds));
    // styled_display_lines.push(display_lines[1].clone().style(style.description));


    let display_name_row = Paragraph::new(display_lines[0].clone().style(style.keybinds));
    let display_description = Paragraph::new(display_lines[1].clone().style(style.description));

    frame.render_widget(display_name_row, keybind_name_row);
    frame.render_widget(display_description, description_row);
}
