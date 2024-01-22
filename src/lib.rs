use ratatui::style::Color;

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;
// Help window
pub mod ui_help;
// Sidebar window
pub mod ui_sidebar;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

/// Utility objects
pub mod utils;

const TOOLBOX_WIDTH: u16 = 30;
const BRUSH_MIN: u16 = 1;
const BRUSH_MAX: u16 = 21;

const BLOCK: &str = "▐█▌";
const LOWER_BLOCK: &str = "▗▄▖";
const UPPER_BLOCK: &str = "▝▀▘";

const BG: Color = Color::Rgb(124, 124, 124);
const BG_DARK: Color = Color::Rgb(96, 96, 96);
// const BG_LIGHT: Color = Color::Rgb(180, 180, 180);
const LAYER_SELECTED: Color = Color::Rgb(75, 75, 75);
const TOOL_BORDER: Color = Color::Rgb(216, 217, 218);
const BUTTON_COLOR: Color = Color::Rgb(127, 187, 179);
const DARK_TEXT: Color = Color::Rgb(35, 42, 46);
const BUTTON_COLOR_SEL: Color = Color::Rgb(255, 248, 227);
