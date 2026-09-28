// TODO: add truncating script name/command when not enough space
//       add border around center section
//       highlight border (aqua?) of section that is selected
//       add label on top of the column
//       handle expansion of packages in tui
//       handle selection highlighting
//       make navigation up/down and searching
//       add running the selected command
//       add displaying the keybinds available

use ratatui::Frame;

use crate::tui::{app::App, rendering::{search_bar::render_search_bar, styling::get_area_with_border, ui_layout::get_ui_layout, ui_rows::render_ui_rows}};

pub mod ui_layout;
pub mod ui_keybinds;
pub mod ui_rows;
pub mod scrolling;
pub mod search_bar;
pub mod styling;


// enum UiRow {
//     Package {
//         id: usize
//     },
//     Script {
//         package_id: usize,
//         id: usize
//     },
//     Vspace
// }

// struct UiLayout {
//     search: Rect,
//     main: Rect,
//     keybind_display: Rect
// }

// struct HighlightedStyles {
//     name_column: Style,
//     command_column: Style
// }


pub fn render(frame: &mut Frame, app: &App) -> () {
    let layout = get_ui_layout(frame.area());

    let main_section = get_area_with_border(frame, layout.main);

    render_search_bar(frame, layout.search, app);
    render_ui_rows(frame, main_section, app);
}




// fn is_package_expanded(package: &Package, expanded_set: &HashSet<usize>) -> bool {
//     return expanded_set.contains(&package.id);
// }

// fn get_visible_rows(app: &App) -> Vec<UiRow> {
//     let mut visible_rows = Vec::new();

//     for package in &app.workspace.packages {
//         let package_ui_row = UiRow::Package { id: package.id };
//         visible_rows.push(package_ui_row);

//         if is_package_expanded(package, &app.expanded_packages) {
            
//             for script in &package.scripts {
//                 let script_ui_row = UiRow::Script { package_id: package.id, id: script.id };
//                 visible_rows.push(script_ui_row);
//             }

//             // add empty row
//             visible_rows.push(UiRow::Vspace);
//         }
//     }

//     return visible_rows;
// }




// fn create_layout(area: Rect) -> UiLayout {
//     let sections = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Length(3),
//             Constraint::Min(1),
//             Constraint::Length(1)
//         ]).split(area);

//     // let main = Layout::default()
//     //     .direction(Direction::Horizontal)
//     //     .constraints([
//     //         Constraint::Percentage(40),
//     //         Constraint::Percentage(60)
//     //     ]).split(vertical[1]);

//     let layout = UiLayout {
//         search: sections[0],
//         main: sections[1],
//         keybind_display: sections[2]
//     };

//     return layout;
// }

// fn is_selected()

// fn get_selected_style()

// fn get_unselected_style()

// // style
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

// render data
// fn render_search_bar(frame: &mut Frame, area: Rect, app: &App) -> () { // TODO: add style parameter
//     let search_widget = Paragraph::new(app.search_query.to_string()).block(
//         Block::default().borders(Borders::ALL).title(" Search ")
//     );

//     frame.render_widget(search_widget, area);
// }

// fn render_package_row(frame: &mut Frame, area: Rect, app: &App, package: &Package) -> () { // TODO: add style parameter
//     let [name_column_area, command_column_area] = Layout::horizontal(
//         [Constraint::Percentage(35), Constraint::Percentage(65)]
//     ).areas(area);

//     let expanded = app.expanded_packages.contains(&package.id);
//     let expanded_indicator = if expanded { "▼" } else { "▶" };
//     let displayed_package_name = format!("{expanded_indicator} {}", package.name);
//     let package_name_widget = Paragraph::new(displayed_package_name);
//     frame.render_widget(package_name_widget, name_column_area);
// }

// fn render_script_row(frame: &mut Frame, area: Rect, app: &App, script: &Script) -> () { // TODO: add style parameter
//     let [name_column_area, command_column_area] = Layout::horizontal(
//         [Constraint::Percentage(35), Constraint::Percentage(65)]
//     ).areas(area);

//     // script name
//     let displayed_script_name = format!("{}", script.name);
//     let script_name_widget = Paragraph::new(format!("   {displayed_script_name}"));
//     frame.render_widget(script_name_widget, name_column_area);

//     // script command
//     let displayed_script_command = format!("{}", script.command);
//     let script_command_widget = Paragraph::new(displayed_script_command);
//     frame.render_widget(script_command_widget, command_column_area);
// }

// fn render_vspace_row(frame: &mut Frame, area: Rect) -> () {
//     let vspace_widget = Paragraph::new("");
//     frame.render_widget(vspace_widget, area);
// }

// fn render_row(frame: &mut Frame, area: Rect, app: &App, ui_row: &UiRow) -> () {
//     match ui_row {
//         UiRow::Package { id } => {
//             let package_wrapped = app.workspace.packages.iter().find(|p| { p.id == *id });
//             if package_wrapped.is_some() {
//                 let package = package_wrapped.unwrap();
//                 render_package_row(frame, area, app, package);
//             }
//         },
//         UiRow::Script { package_id, id } => {
//             let package_wrapped = app.workspace.packages.iter().find(|p| { p.id == *package_id });
//             if package_wrapped.is_none() {
//                 // it is fine to return here, this returns from the function but there is nothing after the match statement
//                 return;
//             };

//             let package = package_wrapped.unwrap();
//             let script_wrapped = package.scripts.iter().find(|s| { s.id == *id });

//             if script_wrapped.is_none() {
//                 return;
//             }

//             let script = script_wrapped.unwrap();

//             render_script_row(frame, area, app, script);

//         },
//         UiRow::Vspace => {
//             render_vspace_row(frame, area);
//         }
//     }
// }

// fn render_rows(frame: &mut Frame, area: Rect, app: &App) -> () {
//     let rows_to_be_rendered = get_visible_rows(app);
//     let mut constraints: Vec<Constraint> = Vec::new();

//     for _ in &rows_to_be_rendered {
//         constraints.push(Constraint::Length(1));
//     }

//     let row_layout = Layout::vertical(constraints);
//     let row_areas = row_layout.split(area);
//     let row_count = rows_to_be_rendered.len();

//     for i in 0..(row_count) {
//         let row_area = row_areas[i];
//         let ui_row = &rows_to_be_rendered[i];
//         render_row(frame, row_area, app, &ui_row);
//     }
// }

// // fn render_package_scripts(frame: &mut Frame, area: Rect, package: &Package, expanded: bool) -> () {
// //     let [name_column_area, command_column_area] = Layout::horizontal(
// //         [Constraint::Percentage(35), Constraint::Percentage(65)]
// //     ).areas(area);

// //     // render package name
// //     let expanded_indicator = if expanded { "v ▼" } else { "> ▶" };
// //     let displayed_package_name = format!("{expanded_indicator} {}", package.name.clone());
// //     let package_name_widget = Paragraph::new(displayed_package_name);
// //     frame.render_widget(package_name_widget, name_column_area);

// //     if expanded {
        
// //     }
// // }

// fn render_view_area(frame: &mut Frame, area: Rect, app: &App) -> () {
//     // for package in &app.workspace.packages {
//     //     let expanded = app.expanded_packages.contains(&package.id);
//     //     render_package_scripts(frame, area, &package, expanded);
//     // }
    
//     render_rows(frame, area, app);
// }

// fn get_area_with_border(frame: &mut Frame, area: Rect) -> Rect {
//     // add border on area
//     let border = Block::default().borders(Borders::ALL).title(" Scripts ");
//     frame.render_widget(border, area);

//     let inner_area = area.inner(Margin { vertical: 1, horizontal: 1 });
//     return inner_area;
// }

// pub fn render(frame: &mut Frame, app: &App) -> () {
//     let layout = create_layout(frame.area());

//     let main_section = get_area_with_border(frame, layout.main);

//     render_search_bar(frame, layout.search, app);
//     render_view_area(frame, main_section, app);
// }
