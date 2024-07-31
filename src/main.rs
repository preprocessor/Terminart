use term_art::app::{App, Result};
use term_art::handler::{handle_key_events, handle_mouse_events};
use term_art::handler::{Event, EventHandler};
use term_art::tui::Tui;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(width, height) => app.resize(width, height),
            Event::Paste(s) => {
                // Take the first char from the clipboard and use it as the brush
                if let Some(c) = s.chars().next() {
                    app.brush.char = c;
                }
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
