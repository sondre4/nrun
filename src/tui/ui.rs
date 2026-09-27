use std::io;

use crossterm::event;

use crate::{tui::{self, app::App, events, terminal::{self, initialize_terminal}}, workspace::types::Workspace};



pub fn render() {

}

pub fn run_tui(workspace: Workspace) -> Result<(), String> {

    // steps:
    // render tui
    // read events
    // handle events
    // update app state

    let mut terminal = initialize_terminal()?;
    let mut app = App::new(workspace);

    // main loop
    while app.running {

        // render
        // terminal.draw(|frame| {
        // });

        // read events
        let event = events::next()?;

        // event to action
        let action = events::to_action(event);

        // handle action

        // update app state
    }

    tui::terminal::restore_terminal();

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
