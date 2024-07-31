use ratatui::style::Color;

use super::{input::color::TextFocus, tools::Tool};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClickAction {
    Draw,
    Prev(Increment),
    Next(Increment),
    Set(SetValue),
    Layer(LayerAction),
    Rename(RenameAction),
    PickColor(PickAction),
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
    Reset(ResetValue),
    Char(char), // 🦎🔥
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetValue {
    FG,
    BG,
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
pub enum RenameAction {
    Accept,
    Nothing,
    Exit,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickAction {
    AcceptFG,
    AcceptBG,
    ReplacePColor(Color, usize),
    Plus(TextFocus),
    Minus(TextFocus),
    ChangeFocus(TextFocus),
    Update(TextFocus, u8),
    New,
    Exit,
    Nothing,
}
