//! Event handling for the TUI.

use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};

/// Events that can occur in the application.
pub enum Event {
    /// Input event (keyboard, mouse, etc.)
    Input(KeyEvent),
    /// Tick event for updating the UI
    Tick,
}

/// Handles events from the terminal.
pub struct EventHandler {
    /// Event sender channel
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    /// Event receiver channel
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Creates a new event handler with the specified tick rate.
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(Duration::from_secs(0));

                    if event::poll(timeout).expect("Failed to poll for events") {
                        if let CrosstermEvent::Key(key) = event::read().expect("Failed to read event") {
                            if let Err(_) = sender.send(Event::Input(key)) {
                                return;
                            }
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        if let Err(_) = sender.send(Event::Tick) {
                            return;
                        }
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Gets the next event from the handler.
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }
}