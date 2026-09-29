use std::io;

use crossterm::event;
use tui::rendering;

use crate::{tui::{self, app::{App}, events, terminal::{self, initialize_terminal}}, workspace::types::Workspace};

pub fn run_tui(workspace: Workspace) -> Result<(), String> {

    // steps:
    // render tui
    // read events
    // handle events
    // update app ui_state

    let mut terminal = initialize_terminal()?;
    let mut app = App::new(workspace);

    app.initialize_expanded_packages();

    // main loop
    while app.running {

        // render
        let render_result = terminal.draw(|frame| {
            rendering::render(frame, &app);
        });

        match render_result {
            Ok(result) => (),
            Err(error) => {
                tui::terminal::restore_terminal()?;
                return Err(format!("Error whilst drawing rendered frame: {error}"));
            }
        }

        // read events
        let event = events::next()?;

        // event to action
        let action_option = events::to_action(app.ui_state, event);

        // handle action
        if action_option.is_some() {
            let action = action_option.unwrap();
            let mapped_app_operation = app.action_to_operation(action);
            app.handle_operation(mapped_app_operation);
        }

        if !app.running {
            break;
        }

        // update app ui_state => happens in app.handle_action(...)
    }

    tui::terminal::restore_terminal()?;

    // loop {
    //     terminal.draw(|frame| {
    //         frame.render_widget("hello", frame.area());
    //     });

    //     // if event::poll(std::time::Duration::from_millis(100)) {
    //     //     if let Event::Key(key) = event::read() {
                
    //     //     }
    //     // }
    // }

    return Ok(());
}
