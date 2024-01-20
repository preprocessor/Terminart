use ratatui::style::{Color, Style};

use super::{cell::Cell, tools::Tool};

#[derive(Clone, Copy, Debug)]
pub struct Brush {
    pub size: u16,
    pub fg: Color,
    pub bg: Color,
    pub char: char,
    pub tool: Tool,
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            size: 1,
            fg: Color::Black,
            bg: Color::White,
            char: '░',
            tool: Tool::default(),
        }
    }
}

impl Brush {
    pub const fn style(&self) -> Style {
        Style::new().fg(self.fg).bg(self.bg)
    }

    pub const fn as_cell(&self) -> Cell {
        Cell {
            fg: self.fg,
            bg: self.bg,
            char: self.char,
        }
    }

    #[rustfmt::skip]    pub fn char(&self) -> String { self.char.to_string() }
}
