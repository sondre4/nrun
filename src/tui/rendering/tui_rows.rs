use ratatui::{Frame, layout::{Constraint, Layout, Rect}, text::{Line, Span}, widgets::Paragraph};

use crate::{tui::{app::App, rendering::{style_elements::{get_package_row_style, get_script_row_style}, visibility::is_package_expanded}, styles::ui_style::{PackageUiStyle, ScriptUiStyle}}, workspace::types::{Package, Script}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiRow {
    Package {
        package_id: usize
    },
    Script {
        package_id: usize,
        script_id: usize
    },
    Vspace
}

pub struct TuiRowData {
    pub package_id: usize,
    pub script_id: Option<usize>
}

impl TuiRow {
    pub fn get_package_id(&self) -> Option<usize> {
        match self {
            TuiRow::Package { package_id } => Some(*package_id),
            TuiRow::Script { package_id, script_id: _ } => Some(*package_id),
            TuiRow::Vspace => None,
        }
    }

    pub fn get_script_id(&self) -> Option<usize> {
        match self {
            TuiRow::Package { package_id: _ } => None,
            TuiRow::Script { package_id: _, script_id } => Some(*script_id),
            TuiRow::Vspace => None,
        }
    }

    pub fn get_row_data(&self) -> Option<TuiRowData> {
        let mut found_package_id: Option<usize> = None;
        let mut found_script_id: Option<usize> = None;

        match self {
            TuiRow::Package { package_id } => {
                found_package_id = Some(*package_id);
            },
            TuiRow::Script { package_id, script_id } => {
                found_package_id = Some(*package_id);
                found_script_id = Some(*script_id);
            },
            TuiRow::Vspace => {
                // return Err(format!("Unable to get row data: row is of type Vspace"));
                return None;
            },
        }

        let data: TuiRowData = TuiRowData {
            package_id: found_package_id.unwrap(),
            script_id: found_script_id
        };

        return Some(data);
    }
}

fn render_package_row(frame: &mut Frame, area: Rect, package_style: PackageUiStyle, package: &Package, expanded: bool) -> () {
    let [name_column_area, _command_column_area] = Layout::horizontal(
        [Constraint::Percentage(35), Constraint::Percentage(65)]
    ).areas(area);

    let expanded_indicator = if expanded { "▼" } else { "▶" };
    let package_name_span = Span::from(format!("{expanded_indicator} {}", package.name)).style(package_style.name);
    let script_count_span = Span::from(format!(" ({})", package.scripts.len())).style(package_style.script_count);

    let mut display_content: Line = Line::default();
    display_content.push_span(package_name_span);
    display_content.push_span(script_count_span);

    let package_name_widget = Paragraph::new(display_content);

    frame.render_widget(package_name_widget, name_column_area);
}

fn render_script_row(frame: &mut Frame, area: Rect, script_style: ScriptUiStyle, script: &Script) -> () {
    let [indent_column, name_column_area, command_column_area] = Layout::horizontal([
        Constraint::Percentage(3),
        Constraint::Percentage(32),
        Constraint::Percentage(65)
    ]).areas(area);

    // render column for whole line background when selected
    frame.render_widget(Paragraph::new("").style(script_style.name), indent_column);

    // script name
    let displayed_script_name = format!("{}", script.name);
    let script_name_widget = Paragraph::new(format!("{displayed_script_name}"))
        .style(script_style.name);

    frame.render_widget(script_name_widget, name_column_area);

    // script command
    let displayed_script_command = format!("{}", script.command);
    let script_command_widget = Paragraph::new(displayed_script_command)
        .style(script_style.command);

    frame.render_widget(script_command_widget, command_column_area);
}

fn render_vspace_row(frame: &mut Frame, area: Rect) -> () {
    let vspace_widget = Paragraph::new("");
    frame.render_widget(vspace_widget, area);
}

fn render_row(frame: &mut Frame, area: Rect, app: &App, tui_row: &TuiRow) -> () {
    match tui_row {
        TuiRow::Package { package_id } => {
            let package_wrapped = app.find_package_by_id(*package_id);

            if package_wrapped.is_none() {
                return;
            }

            let current_view_data = app.get_current_view_data();
            let package = package_wrapped.unwrap();
            let ui_style = get_package_row_style(tui_row, &current_view_data.selection, app.get_styles());

            let expanded = is_package_expanded(&package, &current_view_data.expanded_set);
            render_package_row(frame, area, ui_style, &package, expanded);
        },

        TuiRow::Script { package_id, script_id } => {
            let package_wrapped = app.find_package_by_id(*package_id);
            if package_wrapped.is_none() {
                // it is fine to return here, this returns from the function but there is nothing after the match statement
                return;
            };

            let current_view_data = app.get_current_view_data();
            let package = package_wrapped.unwrap();
            let script_wrapped = app.find_script_by_id(package, *script_id);

            if script_wrapped.is_none() {
                return;
            }

            let script = script_wrapped.unwrap();

            let ui_style = get_script_row_style(tui_row, &current_view_data.selection, app.get_styles());
            render_script_row(frame, area, ui_style, &script);
        },

        TuiRow::Vspace => {
            render_vspace_row(frame, area);
        }
    }
}

pub fn render_ui_rows(frame: &mut Frame, area: Rect, app: &App, visible_rows: &Vec<TuiRow>) -> () {
    let mut constraints: Vec<Constraint> = Vec::new();

    for _ in visible_rows {
        constraints.push(Constraint::Length(1));
    }

    let row_layout = Layout::vertical(constraints);
    let row_areas = row_layout.split(area);
    let row_count = visible_rows.len();

    for i in 0..(row_count) {
        let row_area = row_areas[i];
        let visible_row = &visible_rows[i];
        render_row(frame, row_area, app, visible_row);
    }
}
