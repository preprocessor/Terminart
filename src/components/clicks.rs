use ratatui::style::Color;

use super::{input::color::TextFocus, tools::Tools};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClickAction {
    Draw,
    Prev(Increment),
    Next(Increment),
    Set(SetValue),
    Layer(LayerAction),
    Rename(PopupBoxAction),
    Export(PopupBoxAction),
    Save(PopupBoxAction),
    Exit(PopupBoxAction),
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
    Tool(Tools),
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
pub enum PopupBoxAction {
    Accept,
    Nothing,
    Deny,
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
