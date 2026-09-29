use ratatui::{Frame, layout::{Constraint, Layout, Rect}, widgets::Paragraph};

use crate::{tui::{app::App, rendering::{visibility::build_visible_rows, styling::{PackageUiStyle, ScriptUiStyle, get_package_row_style, get_script_row_style}}}, workspace::types::{Package, Script}};

#[derive(Debug, Clone)]
pub enum UiRow {
    Package {
        package_id: usize
    },
    Script {
        package_id: usize,
        script_id: usize
    },
    Vspace
}




fn render_package_row(frame: &mut Frame, area: Rect, package_style: PackageUiStyle, app: &App, package: &Package) -> () {
    let [name_column_area, command_column_area] = Layout::horizontal(
        [Constraint::Percentage(35), Constraint::Percentage(65)]
    ).areas(area);

    let expanded = app.expanded_packages.contains(&package.id);
    let expanded_indicator = if expanded { "▼" } else { "▶" };
    let displayed_package_name = format!("{expanded_indicator} ({}) {}", package.id, package.name);

    let package_name_widget = Paragraph::new(displayed_package_name)
        .style(package_style.name);

    frame.render_widget(package_name_widget, name_column_area);
}

fn render_script_row(frame: &mut Frame, area: Rect, script_style: ScriptUiStyle, app: &App, script: &Script) -> () {
    let [indent_column, name_column_area, command_column_area] = Layout::horizontal([
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

fn render_row(frame: &mut Frame, area: Rect, app: &App, ui_row: &UiRow) -> () {
    match ui_row {
        UiRow::Package { package_id } => {
            let package_wrapped = app.workspace.packages.iter().find(|p| { p.id == *package_id });

            if package_wrapped.is_some() {
                let package = package_wrapped.unwrap();
                let ui_style = get_package_row_style(package, &app.selection);

                render_package_row(frame, area, ui_style, app, package);
            }
        },
        UiRow::Script { package_id, script_id } => {
            let package_wrapped = app.workspace.packages.iter().find(|p| { p.id == *package_id });
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

            let ui_style = get_script_row_style(script, &app.selection);
            render_script_row(frame, area, ui_style, app, script);

        },
        UiRow::Vspace => {
            render_vspace_row(frame, area);
        }
    }
}

pub fn render_ui_rows(frame: &mut Frame, area: Rect, app: &App) -> () {
    let rows_to_be_rendered = build_visible_rows(app);
    let mut constraints: Vec<Constraint> = Vec::new();

    for _ in &rows_to_be_rendered {
        constraints.push(Constraint::Length(1));
    }

    let row_layout = Layout::vertical(constraints);
    let row_areas = row_layout.split(area);
    let row_count = rows_to_be_rendered.len();

    for i in 0..(row_count) {
        let row_area = row_areas[i];
        let ui_row = &rows_to_be_rendered[i];
        render_row(frame, row_area, app, ui_row);
    }
}
