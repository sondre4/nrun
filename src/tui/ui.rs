use crate::{tui::{self, app::App, events, operation::OperationResult, rendering::tui_view::TuiView, terminal::{TerminalGuard, initialize_terminal}}, workspace::types::Workspace};

pub fn run_tui(workspace: Workspace) -> Result<(), String> {

    // steps:
    // read events
    // handle events
    // update app
    // render tui

    // initialize terminal
    let terminal = initialize_terminal()?;
    let mut terminal_guard = TerminalGuard::new(terminal);
    let terminal_mut_ref = terminal_guard.get_terminal_mut();

    let mut app = App::new(workspace);
    let mut last_operation_result: OperationResult = OperationResult::None;
    // let mut searcher: PackageSearcher = PackageSearcher::new();

    app.initialize_expanded_packages();

    // main loop
    while app.running() {

        // read events
        let event = events::next()?; // blocks thread

        // event to action
        let new_action = events::to_action(app.get_focused_section(), event);

        // handle action
        if new_action.is_some() {
            let action = new_action.unwrap();
            let mapped_app_operation = app.action_to_operation(action);
            last_operation_result = app.handle_operation(mapped_app_operation);
        }

        if !app.running() {
            break;
        }

        // Need to initialize the search expanded packages before updating filtered rows
        app.prepare_searching_expanding_packages_if_needed(&last_operation_result);
        app.update_filtered_rows();
        app.prepare_current_selection_if_needed(&last_operation_result);


        // Render after updating state
        let current_filtered_rows = app.get_filtered_rows();
        let mut view: TuiView = TuiView::initialize_new(current_filtered_rows);

        let render_result = terminal_mut_ref.draw(|frame| {
            let current_view_data = app.get_current_view_data();
            let current_focused_section = app.get_focused_section();

            // initialize layout
            let mut layout = view.get_tui_layout(frame);
            let color_palette = app.get_color_palette();
            layout = view.style_tui_layout(frame, layout, current_focused_section, color_palette);

            // calculate viewport values
            let viewport_state = view.calculate_viewport_state(
                &layout,
                &current_view_data.selection,
                current_view_data.scroll_offset,
                &last_operation_result
            );

            app.update_current_scroll_offset(viewport_state.scroll_offset);

            // render
            view.render(frame, layout, viewport_state, &app);
        });

        match render_result {
            Ok(_result) => (),
            Err(error) => {
                terminal_guard.restore();
                return Err(format!("Error whilst rendering: {error}"));
            }
        }
    }

    terminal_guard.restore();

    app.handle_operation_result(&last_operation_result)?;

    return Ok(());
}
