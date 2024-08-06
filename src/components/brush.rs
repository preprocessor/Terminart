use ratatui::style::{Color, Style};
use serde::{Deserialize, Serialize};

use super::{cell::Cell, tools::Tools};

const BRUSH_MIN: u16 = 1;
const BRUSH_MAX: u16 = 21;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Brush {
    pub fg: Color,
    pub bg: Color,

    pub size: u16,
    pub char: char,
    pub tool: Tools,
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            size: 1,
            fg: Color::Black,
            bg: Color::White,
            char: '░',
            tool: Tools::default(),
        }
    }
}

impl Brush {
    pub const fn style(&self) -> Style {
        Style::new().fg(self.fg).bg(self.bg)
    }

    #[rustfmt::skip]    pub fn char(&self) -> String { self.char.to_string() }

    pub const fn as_cell(&self) -> Cell {
        Cell {
            fg: self.fg,
            bg: self.bg,
            char: self.char,
        }
    }

    pub fn down(&mut self, val: u16) {
        self.size = self.size.saturating_sub(val).max(BRUSH_MIN);
    }

    pub fn up(&mut self, val: u16) {
        self.size = (self.size + val).min(BRUSH_MAX);
    }
}
