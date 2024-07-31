use ratatui::style::{Color, Style};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub char: char,
}

impl Cell {
    #[rustfmt::skip]
    pub fn char(&self) -> String { self.char.into() }

    pub const fn style(&self) -> Style {
        Style::new().fg(self.fg).bg(self.bg)
    }
}

/// For use with undo history, for removing a cell use app.erase
impl Default for Cell {
    fn default() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            char: ' ',
        }
    }
}
