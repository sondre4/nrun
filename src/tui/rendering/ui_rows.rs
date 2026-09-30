use ratatui::{Frame, layout::{Constraint, Layout, Rect}, widgets::Paragraph};

use crate::{tui::{app::App, rendering::{styling::{PackageUiStyle, ScriptUiStyle, get_package_row_style, get_script_row_style}, visibility::is_package_expanded}}, workspace::types::{Package, Script}};

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

fn render_package_row(frame: &mut Frame, area: Rect, package_style: PackageUiStyle, app: &App, package: &Package) -> () {
    let [name_column_area, _command_column_area] = Layout::horizontal(
        [Constraint::Percentage(35), Constraint::Percentage(65)]
    ).areas(area);

    let expanded = is_package_expanded(package, &app.app_state.expanded_packages);
    let expanded_indicator = if expanded { "▼" } else { "▶" };
    let displayed_package_name = format!("{expanded_indicator} ({}) {}", package.id, package.name);

    let package_name_widget = Paragraph::new(displayed_package_name)
        .style(package_style.name);

    frame.render_widget(package_name_widget, name_column_area);
}

fn render_script_row(frame: &mut Frame, area: Rect, script_style: ScriptUiStyle, script: &Script) -> () {
    let [_indent_column, name_column_area, command_column_area] = Layout::horizontal([
        Constraint::Percentage(3),
        Constraint::Percentage(32),
        Constraint::Percentage(65)
    ]).areas(area);

    // script name
    let displayed_script_name = format!("{}", script.name);
    let script_name_widget = Paragraph::new(format!("({}) {displayed_script_name}", script.id))
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
            let package_wrapped = app.app_state.workspace.packages.iter().find(|p| { p.id == *package_id });

            if package_wrapped.is_some() {
                let package = package_wrapped.unwrap();
                let ui_style = get_package_row_style(tui_row, &app.app_state.selection);

                render_package_row(frame, area, ui_style, app, package);
            }
        },
        TuiRow::Script { package_id, script_id } => {
            let package_wrapped = app.app_state.workspace.packages.iter().find(|p| { p.id == *package_id });
            if package_wrapped.is_none() {
                // it is fine to return here, this returns from the function but there is nothing after the match statement
                return;
            };

            let package = package_wrapped.unwrap();
            let script_wrapped = package.scripts.iter().find(|s| { s.id == *script_id });

            if script_wrapped.is_none() {
                return;
            }

            let script = script_wrapped.unwrap();

            let ui_style = get_script_row_style(tui_row, &app.app_state.selection);
            render_script_row(frame, area, ui_style, script);

        },
        TuiRow::Vspace => {
            render_vspace_row(frame, area);
        }
    }
}

pub fn render_ui_rows(frame: &mut Frame, area: Rect, app: &App, visible_rows: &Vec<TuiRow>) -> () {
    // let visible_rows = build_visible_rows(app);

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
