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
pub mod utils_charpicker;
pub mod utils_shapes;

const TOOLBOX_WIDTH: u16 = 30;
const BRUSH_MIN: u16 = 1;
const BRUSH_MAX: u16 = 21;

const BLOCK: &str = "▐█▌";
const LOWER_BLOCK: &str = "▗▄▖";
const UPPER_BLOCK: &str = "▝▀▘";

const TOOLBOX_BG: Color = Color::Rgb(124, 124, 124);
const TOOL_BORDER: Color = Color::Rgb(216, 217, 218);
const BUTTON_COLOR: Color = Color::Rgb(127, 187, 179);
const BUTTON_TEXT: Color = Color::Rgb(35, 42, 46);
const BUTTON_COLOR_SEL: Color = Color::Rgb(255, 248, 227);
const BUTTON_TEXT_SEL: Color = Color::Rgb(80, 60, 60);
