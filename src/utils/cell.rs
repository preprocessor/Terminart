use ratatui::style::{Color, Style};

#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub char: char,
}

impl Cell {
    #[rustfmt::skip]    pub fn char(&self) -> String { self.char.to_string() }

    pub const fn style(&self) -> Style {
        Style::new().fg(self.fg).bg(self.bg)
    }

    /// For use with undo history, for removing a cell use app.erase
    pub const fn empty() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            char: ' ',
        }
    }
}
