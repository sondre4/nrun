use crate::{search::searcher::PackageSearcher, tui::{self, app::App, events, operation::OperationResult, rendering::tui_view::TuiView, terminal::initialize_terminal}, workspace::types::Workspace};

// TODO: refactor to:
// 1. wait for event
// 2. read/handle event
// 3. update app state
// 4. render

// this should solve both 

pub fn run_tui(workspace: Workspace) -> Result<(), String> {

    // steps:
    // render tui
    // read events
    // handle events
    // update app

    let mut terminal = initialize_terminal()?;
    let mut app = App::new(workspace);
    let mut last_operation_result: OperationResult = OperationResult::None;
    let mut searcher: PackageSearcher = PackageSearcher::new();

    app.initialize_expanded_packages();

    // main loop
    while app.app_state.running {

        // filter the data based on search query
        let packages = &app.app_state.workspace.packages;
        let search_results = searcher.search(packages, app.app_state.search_input.value());

        // initialize tui_view
        let mut view: TuiView = TuiView::initialize_new(&app, &search_results);

        // get a clone of the view's filtered rows and update app state filtered rows
        // app state needs them for navigating handling
        let filtered_rows = view.get_filtered_tui_rows();
        app.tui_view_data.filtered_rows = filtered_rows;


        let render_result = terminal.draw(|frame| {

            // initialize layout
            let mut layout = view.get_tui_layout(frame);
            layout = view.style_tui_layout(frame, layout, &app.app_state.focused_section);

            // calculate viewport values
            let viewport_state = view.calculate_viewport_state(
                &layout,
                &app.get_current_selection(),
                app.tui_view_data.current_scroll_offset,
                &last_operation_result
            );

            // update app state scroll offset
            app.tui_view_data.current_scroll_offset = viewport_state.scroll_offset;

            // render
            view.render(frame, layout, viewport_state, &app);
        });

        match render_result {
            Ok(_result) => (),
            Err(error) => {
                tui::terminal::restore_terminal()?;
                return Err(format!("Error whilst rendering: {error}"));
            }
        }

        // read events
        let event = events::next()?;

        // event to action
        let action_option = events::to_action(app.app_state.focused_section, event);

        // handle action
        if action_option.is_some() {
            let action = action_option.unwrap();
            let mapped_app_operation = app.action_to_operation(action);
            last_operation_result = app.handle_operation(mapped_app_operation);
        }

        if !app.app_state.running {
            break;
        }

        // update app focused_section => happens in app.handle_action(...)
    }

    tui::terminal::restore_terminal()?;

    return Ok(());
}
