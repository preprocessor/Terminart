use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Palette {
    pub colors: Vec<Color>,

    fg: usize,
    bg: usize,
}

impl Default for Palette {
    fn default() -> Self {
        let colors = vec![
            Color::Black,        //  0
            Color::Red,          //  1
            Color::Yellow,       //  2
            Color::Green,        //  3
            Color::Blue,         //  4
            Color::Magenta,      //  5
            Color::Cyan,         //  6
            Color::Gray,         //  7
            Color::DarkGray,     //  8
            Color::LightRed,     //  9
            Color::LightYellow,  // 10
            Color::LightGreen,   // 11
            Color::LightBlue,    // 12
            Color::LightMagenta, // 13
            Color::LightCyan,    // 14
            Color::White,        // 15
        ];
        Self {
            colors,
            fg: 0,
            bg: 15,
        }
    }
}

impl Palette {
    pub fn colors(&self) -> Vec<Color> {
        self.colors.to_vec()
    }

    fn next(&self, index: usize) -> usize {
        (index + 1) % self.colors.len()
    }

    fn prev(&self, index: usize) -> usize {
        index.checked_sub(1).unwrap_or(self.colors.len() - 1)
    }

    pub fn replace(&mut self, index: usize, color: Color) {
        if index > 15 {
            return;
        }

        self.colors[index] = color;
    }

    pub fn fg_next(&mut self) -> Color {
        self.fg = self.next(self.fg);
        self.colors[self.fg]
    }

    pub fn fg_prev(&mut self) -> Color {
        self.fg = self.prev(self.fg);
        self.colors[self.fg]
    }

    pub fn bg_next(&mut self) -> Color {
        self.bg = self.next(self.bg);
        self.colors[self.bg]
    }

    pub fn bg_prev(&mut self) -> Color {
        self.bg = self.prev(self.bg);
        self.colors[self.bg]
    }
}
