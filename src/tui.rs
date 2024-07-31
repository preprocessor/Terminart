use crate::app::{App, Result};
use crate::handler::EventHandler;
use crate::ui;
use better_panic::Settings;
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            EnterAlternateScreen,
            EnableMouseCapture,
            EnableBracketedPaste
        )?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        std::panic::set_hook(Box::new(|panic_info| {
            #[allow(clippy::expect_used)]
            Self::reset().expect("failed to reset the terminal");
            Settings::auto()
                .most_recent_first(false)
                .lineno_suffix(true)
                .create_panic_handler()(panic_info);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }
    // pub fn init(&mut self) -> Result<()> {
    //     terminal::enable_raw_mode()?;
    //     crossterm::execute!(
    //         io::stderr(),
    //         EnterAlternateScreen,
    //         EnableMouseCapture,
    //         EnableBracketedPaste
    //     )?;
    //
    //     // Define a custom panic hook to reset the terminal properties.
    //     // This way, you won't have your terminal messed up if an unexpected error happens.
    //     let panic_hook = panic::take_hook();
    //     panic::set_hook(Box::new(move |panic| {
    //         #[allow(clippy::expect_used)]
    //         Self::reset().expect("failed to reset the terminal");
    //         panic_hook(panic);
    //     }));
    //
    //     self.terminal.hide_cursor()?;
    //     self.terminal.clear()?;
    //     Ok(())
    // }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: ratatui::Terminal::draw
    /// [`rendering`]: crate::ui:render
    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            DisableBracketedPaste
        )?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
