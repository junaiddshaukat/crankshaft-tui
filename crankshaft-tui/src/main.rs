use std::time::Duration;
use crankshaft_tui::{App, init_terminal, restore_terminal, run_app};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the terminal
    let mut terminal = init_terminal()?;
    
    // Create the application state
    let mut app = App::new();
    
    // Run the application with a tick rate of 250ms
    run_app(&mut terminal, &mut app, Duration::from_millis(250))?;
    
    // Restore the terminal
    restore_terminal(&mut terminal)?;
    
    Ok(())
}