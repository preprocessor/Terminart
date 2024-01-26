use ratatui::style::Color;

use super::tools::Tool;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClickAction {
    Draw,
    Prev(Increment),
    Next(Increment),
    Set(SetValue),
    Layer(LayerAction),
    Typing(TypingAction),
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Increment {
    CharPicker,
    BrushSize,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetValue {
    Tool(Tool),
    Color(Color),
    Char(char), // 🦎🔥
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayerAction {
    Add,
    Remove,
    Rename,
    Select(u8),
    MoveUp,
    MoveDown,
    ToggleVis(u8),
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypingAction {
    Accept,
    Nothing,
    Exit,
}
