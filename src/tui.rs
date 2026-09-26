use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::workspace::types::Workspace;

pub mod app;
pub mod events;
pub mod searching;
pub mod terminal;
pub mod types;
pub mod ui;

pub fn launch_tui(data: Workspace) -> io::Result<()> {

    // steps:
    // render tui
    // read events
    // handle events
    // update app state



    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen);

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| {
            frame.render_widget("hello", frame.area());
        });

        // if event::poll(std::time::Duration::from_millis(100)) {
        //     if let Event::Key(key) = event::read() {
                
        //     }
        // }
    }

    return Ok(());
}
