use ratatui::style::Color;

use super::tools::Tool;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClickAction {
    None,
    Draw,
    Prev(Increment),
    Next(Increment),
    Set(SetValue),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Increment {
    CharPicker,
    BrushSize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetValue {
    Tool(Tool),
    Color(Color),
    Char(char), // 🦎🔥
}
