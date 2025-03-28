//! Terminal User Interface for monitoring Crankshaft tasks.

mod app;
mod ui;
mod event;

pub use app::{App, Task, TaskStatus};
pub use event::{Event, EventHandler};
pub use ui::draw;

use std::io;
use std::time::Duration;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

/// Initializes the terminal for TUI rendering.
pub fn init_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restores the terminal to its original state.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

/// Runs the TUI application.
// In the run_app function
pub fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut event_handler = EventHandler::new(tick_rate);

    loop {
        terminal.draw(|f| draw(f, app))?;

        // Fix the error handling for the event handler
        match event_handler.next() {
            Ok(Event::Input(key)) => {
                if app.handle_key(key) {
                    break;
                }
            }
            Ok(Event::Tick) => {
app.update();
            }
            Err(err) => {
                // Handle the error appropriately
                eprintln!("Error: {:?}", err);
                break;
            }
            _ => {}
        }
        
        if app.should_quit {
            break;
        }
    }
    
    Ok(())
}