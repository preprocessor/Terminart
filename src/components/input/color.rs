use std::{
    num::{IntErrorKind, ParseIntError},
    time::{Duration, Instant},
};

use ratatui::prelude::Color;

use crate::ui::COLOR_STEP_AMT;

use super::text::TextArea;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum TextFocus {
    #[default]
    Hex,
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub struct ColorPicker {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    // pub buffer: String,
    // pub pos: usize,
    pub text: TextArea,
    pub focus: TextFocus,
    last_update: Instant,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            // buffer: String::default(),
            // pos: 0,
            text: TextArea::default(),
            focus: TextFocus::default(),
            last_update: Instant::now(),
        }
    }
}

impl ColorPicker {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn colors(&self) -> [(u8, TextFocus); 3] {
        [
            (self.r, TextFocus::Red),
            (self.g, TextFocus::Green),
            (self.b, TextFocus::Blue),
        ]
    }

    pub fn pos(&self) -> u16 {
        self.text.pos as u16
    }

    /// Returns a ratatui Color enum of the current color
    pub fn get_style_color(&self) -> Color {
        Color::Rgb(self.r, self.g, self.b)
    }

    pub fn get_hex_str(&self) -> String {
        match self.focus {
            TextFocus::Hex => self.text.buffer.clone(),
            _ => format!("{:02X?}{:02X?}{:02X?}", self.r, self.g, self.b),
        }
    }

    pub fn plus(&mut self, target: TextFocus) {
        match target {
            TextFocus::Hex => {}
            TextFocus::Red => self.r = self.r.saturating_add(COLOR_STEP_AMT),
            TextFocus::Green => self.g = self.g.saturating_add(COLOR_STEP_AMT),
            TextFocus::Blue => self.b = self.b.saturating_add(COLOR_STEP_AMT),
        };
    }

    pub fn minus(&mut self, target: TextFocus) {
        match target {
            TextFocus::Hex => {}
            TextFocus::Red => self.r = self.r.saturating_sub(COLOR_STEP_AMT),
            TextFocus::Green => self.g = self.g.saturating_sub(COLOR_STEP_AMT),
            TextFocus::Blue => self.b = self.b.saturating_sub(COLOR_STEP_AMT),
        };
    }

    pub fn tab(&mut self) {
        self.set_attention(match self.focus {
            TextFocus::Hex => TextFocus::Red,
            TextFocus::Red => TextFocus::Green,
            TextFocus::Green => TextFocus::Blue,
            TextFocus::Blue => TextFocus::Hex,
        });
    }

    pub fn backtab(&mut self) {
        self.set_attention(match self.focus {
            TextFocus::Hex => TextFocus::Blue,
            TextFocus::Red => TextFocus::Hex,
            TextFocus::Green => TextFocus::Red,
            TextFocus::Blue => TextFocus::Green,
        });
    }

    pub fn set(&mut self, target: TextFocus, val: u8) {
        if self.last_update.elapsed() < Duration::from_millis(50) {
            return;
        }

        self.last_update = Instant::now();

        match target {
            TextFocus::Hex => {}
            TextFocus::Red => self.r = val,
            TextFocus::Green => self.g = val,
            TextFocus::Blue => self.b = val,
        };
    }

    /// Read the buffer as a hexidecimal color code and set the r, g, b values accordingly
    fn buf_to_hex(&mut self) {
        let buffer = &mut self.text.buffer;
        *buffer = buffer.replacen('#', "", 1);

        match buffer.len() {
            3 => {
                let mut new_hex = String::with_capacity(6);

                for char in buffer.chars() {
                    new_hex.push(char);
                    new_hex.push(char);
                }

                *buffer = new_hex;

                self.buf_to_hex();
            }
            6 => {
                if buffer.chars().all(|c| c.is_ascii_hexdigit()) {
                    let r = u8::from_str_radix(&buffer[0..=1], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&buffer[2..=3], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&buffer[4..=5], 16).unwrap_or(0);

                    self.r = r;
                    self.g = g;
                    self.b = b;
                }
            }
            _ => {}
        }
    }

    /// Read the buffer as a u8 and returns a result to be used to set the color componenet
    fn buf_to_comp(&mut self) -> Result<u8, ParseIntError> {
        let value = match self.text.buffer.parse::<u8>() {
            Err(e) => match e.kind() {
                IntErrorKind::PosOverflow => Ok(255),
                IntErrorKind::NegOverflow => Ok(0),
                _ => Err(e),
            },
            i => i,
        };

        value
    }

    pub fn update(&mut self) {
        match self.focus {
            TextFocus::Hex => self.buf_to_hex(),
            TextFocus::Red => {
                if let Ok(new_value) = self.buf_to_comp() {
                    self.r = new_value;
                }
            }
            TextFocus::Green => {
                if let Ok(new_value) = self.buf_to_comp() {
                    self.g = new_value;
                }
            }
            TextFocus::Blue => {
                if let Ok(new_value) = self.buf_to_comp() {
                    self.b = new_value;
                }
            }
        }
    }

    pub fn set_attention(&mut self, attn: TextFocus) {
        if self.focus == attn {
            return;
        }

        self.update();

        self.text.clear();

        self.text.buffer = match attn {
            TextFocus::Hex => self.get_hex_str(),
            TextFocus::Red => format!("{}", self.r),
            TextFocus::Green => format!("{}", self.g),
            TextFocus::Blue => format!("{}", self.b),
        };

        self.text.pos = self.text.buffer.len();

        self.focus = attn;
    }

    pub fn input(&mut self, c: char) {
        match self.focus {
            TextFocus::Hex => {
                if !c.is_ascii_hexdigit() {
                    return;
                }
                self.text.input(c, 6);
            }
            _ => {
                if !c.is_ascii_digit() {
                    return;
                }
                self.text.input(c, 6);

                self.update();
            }
        }

        // self.update();
    }
}

// fn is_hex_color(s: &str) -> bool {
//     match s.len() {
//         3 | 6 => s.chars().all(|c| c.is_ascii_hexdigit()),
//         _ => false,
//     }
// }
