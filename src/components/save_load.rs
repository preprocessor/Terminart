use std::fs::File;
use std::io::{BufReader, Read};

use anstyle_parse::{DefaultCharAccumulator, Params, Parser, Perform};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use super::brush::Brush;
use super::cell::Cell;
use super::layers::{Layer, LayerData};
use super::palette::Palette;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub brush: Brush,
    pub palette: Palette,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FileSaveError {
    NoName,
    NoCanvas,
    NameConflict,
    CantCreate,
    Other,
}

pub struct AnsiData;
// adapted from: https://github.com/jwalton/rust-ansi-converter/blob/master/src/ansi_parser.rs

impl AnsiData {
    pub fn open_file(file: File) -> LayerData {
        let mut performer = AnsiParser::default();
        let mut statemachine = Parser::<DefaultCharAccumulator>::new();
        let reader = BufReader::new(file);
        for byte in reader.bytes() {
            statemachine.advance(&mut performer, byte.unwrap());
        }

        performer.output
    }

    pub fn read_str(ansi: String) -> LayerData {
        let mut performer = AnsiParser::default();
        let mut statemachine = Parser::<DefaultCharAccumulator>::new();
        for byte in ansi.bytes() {
            statemachine.advance(&mut performer, byte);
        }

        performer.output
    }
}

struct AnsiParser {
    current_fg: Color,
    current_bg: Color,
    current_x: u16,
    current_y: u16,
    output: LayerData,
}

impl Default for AnsiParser {
    fn default() -> Self {
        Self {
            current_fg: Default::default(),
            current_bg: Default::default(),
            current_x: 1,
            current_y: 1,
            output: Default::default(),
        }
    }
}

impl Perform for AnsiParser {
    fn print(&mut self, c: char) {
        self.output.insert(
            (self.current_x, self.current_y),
            Cell {
                fg: self.current_fg,
                bg: self.current_bg,
                char: c,
            },
        );
        self.current_x += 1;
    }

    fn execute(&mut self, byte: u8) {
        if byte == b'\n' {
            self.current_x = 1;
            self.current_y += 1;
        }
    }

    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, _c: u8) {
        let mut iter = params.iter();
        while let Some(value) = iter.next() {
            match value[0] {
                38 => match iter.next().unwrap()[0] {
                    5 => {
                        let c256 = iter.next().unwrap()[0];
                        self.current_fg = Color::Indexed(c256 as u8)
                    }
                    2 => {
                        let r = iter.next().unwrap()[0] as u8;
                        let g = iter.next().unwrap()[0] as u8;
                        let b = iter.next().unwrap()[0] as u8;
                        self.current_fg = Color::Rgb(r, g, b)
                    }
                    _ => {
                        // panic!("Unknown color mode");
                        return;
                    }
                },
                48 => match iter.next().unwrap()[0] {
                    5 => {
                        let c256 = iter.next().unwrap()[0];
                        self.current_bg = Color::Indexed(c256 as u8)
                    }
                    2 => {
                        let r = iter.next().unwrap()[0] as u8;
                        let g = iter.next().unwrap()[0] as u8;
                        let b = iter.next().unwrap()[0] as u8;
                        self.current_bg = Color::Rgb(r, g, b)
                    }
                    _ => {
                        // panic!("Unknown color mode");
                        return;
                    }
                },
                30..=37 | 39 | 90..=97 => {
                    self.current_fg = match value[0] {
                        30 => Color::Black,
                        31 => Color::Red,
                        32 => Color::Green,
                        33 => Color::Yellow,
                        34 => Color::Blue,
                        35 => Color::Magenta,
                        36 => Color::Cyan,
                        37 => Color::Gray,
                        39 => Color::Reset,
                        90 => Color::DarkGray,
                        91 => Color::LightRed,
                        92 => Color::LightGreen,
                        93 => Color::LightYellow,
                        94 => Color::LightBlue,
                        95 => Color::LightMagenta,
                        96 => Color::LightCyan,
                        97 => Color::White,
                        _ => return,
                    }
                }
                40..=47 | 49 | 100..=107 => {
                    self.current_bg = match value[0] {
                        40 => Color::Black,
                        41 => Color::Red,
                        42 => Color::Green,
                        43 => Color::Yellow,
                        44 => Color::Blue,
                        45 => Color::Magenta,
                        46 => Color::Cyan,
                        47 => Color::Gray,
                        49 => Color::Reset,
                        100 => Color::DarkGray,
                        101 => Color::LightRed,
                        102 => Color::LightGreen,
                        103 => Color::LightYellow,
                        104 => Color::LightBlue,
                        105 => Color::LightMagenta,
                        106 => Color::LightCyan,
                        107 => Color::White,
                        _ => return,
                    }
                }
                0 => {
                    self.current_fg = Color::Reset;
                    self.current_bg = Color::Reset;
                }
                _v => {
                    // panic!("Unhandled color mode {v}");
                    return;
                }
            }
        }
    }
}
