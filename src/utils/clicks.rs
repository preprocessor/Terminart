use ratatui::style::Color;

use super::tools::Tool;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClickAction {
    None,
    Draw,
    Prev(Increment),
    Next(Increment),
    Set(SetValue),
    Layer(LayerAction),
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

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum LayerAction {
//     Add,
//     Remove(String),
//     Select(String),
//     MoveUp,
//     MoveDown,
//     ToggleShow(String),
// }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayerAction {
    Add,
    Remove(usize),
    Select(usize),
    MoveUp(usize),
    MoveDown(usize),
    ToggleShow(usize),
}
