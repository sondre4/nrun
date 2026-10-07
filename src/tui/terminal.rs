use std::io::{self, Write};

use crossterm::{cursor, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Terminal, backend::CrosstermBackend};

pub type AppTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct TerminalGuard {
    terminal: Option<Terminal<CrosstermBackend<io::Stdout>>>,
    terminal_restored: bool
}

impl TerminalGuard {
    pub fn new(terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Self {
        return TerminalGuard {
            terminal: Some(terminal),
            terminal_restored: false
        };
    }

    pub fn get_terminal_mut(&mut self) -> &mut Terminal<CrosstermBackend<io::Stdout>> {
        return self.terminal.as_mut().expect("Terminal has been moved");
    }

    pub fn restore(&mut self) -> () {
        if self.terminal.is_none() {
            return;
        }

        let terminal = self.terminal.as_mut().expect("Terminal has been moved");
        let result = restore_terminal(terminal.backend_mut());

        match result {
            Ok(_) => {
                self.terminal_restored = true;
                return;
            },
            Err(error) => {
                println!("{error}");
            }
        }
    }

    // pub fn take_terminal(&mut self) -> Terminal<CrosstermBackend<io::Stdout>> {
    //     return self.terminal.take().expect("Terminal has already been taken");
    // }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) -> () {
        if self.terminal_restored {
            return;
        }

        let terminal = self.terminal.as_mut().expect("Terminal has been moved");
        let stdout_backend = terminal.backend_mut();
        let result = restore_terminal(stdout_backend);

        match result {
            Ok(_) => {
                return;
            },
            Err(error) => {
                println!("{error}");
            }
        }
    }
}

// when starting TUI
pub fn initialize_terminal() -> Result<AppTerminal, String> {
    let enabling_raw_result = enable_raw_mode();
    match enabling_raw_result {
        Ok(_result) => { },
        Err(error) => {
            return Err(format!("Error whilst enabling raw terminal: {error}"));
        }
    }

    let mut stdout = io::stdout();

    let enter_result = execute!(stdout, EnterAlternateScreen);
    match enter_result {
        Ok(_result) => { },
        Err(error) => {
            return Err(format!("Error whilst entering alternate screen: {error}"));
        }
    }

    let backend = CrosstermBackend::new(stdout);
    let terminal_result = Terminal::new(backend);

    let terminal = match terminal_result {
        Ok(result) => result,
        Err(error) => {
            return Err(format!("Error whilst initializing terminal: {error}"));
        }
    };

    return Ok(terminal);
}

// when TUI closes
pub fn restore_terminal(stdout_backend: &mut CrosstermBackend<io::Stdout>) -> Result<(), String> {
    let disabling_raw_result = disable_raw_mode();
    match disabling_raw_result {
        Ok(_result) => (),
        Err(error) => {
            return Err(format!("Error whilst disabling raw terminal: {error}"));
        }
    }

    let leave_result = execute!(
        stdout_backend,
        LeaveAlternateScreen,
        cursor::Show,
        cursor::MoveToColumn(0),
        cursor::MoveToNextLine(1)
    );

    match leave_result {
        Ok(_result) => { },
        Err(error) => {
            return Err(format!("Error whilst leaving alternate screen: {error}"));
        }
    }

    // let flush_result = stdout.flush();
    // match flush_result {
    //     Ok(_result) => { },
    //     Err(error) => {
    //         return Err(format!("Error whilst leaving alternate screen: {error}"));
    //     }
    // }

    return Ok(());
}
