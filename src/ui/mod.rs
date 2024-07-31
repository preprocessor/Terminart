mod canvas;
mod help;
mod picker;
mod rename;
mod sidebar;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

use crate::app::App;
use crate::utils::input::InputMode;

pub const TOOLBOX_WIDTH: u16 = 30;
pub const _COLOR_STEPS: u8 = 32;
pub const COLOR_STEPS: u8 = _COLOR_STEPS + 1;
pub const COLOR_STEP: u8 = u8::MAX / COLOR_STEPS + 1;

// I am inporting like this to make the struct work with my rgb highlighter
use ratatui::style::Color::{self, Rgb as rgb};

pub const WHITE: Color = rgb(220, 220, 220);
pub const BLACK: Color = rgb(10, 10, 10);
pub const BG: Color = rgb(100, 100, 100);
pub const BG_DARK: Color = rgb(70, 70, 70);
// const BG_LIGHT: Color = rgb(180, 180, 180);
pub const LAYER_SELECTED: Color = rgb(120, 120, 120);
pub const TOOL_BORDER: Color = rgb(240, 240, 240);
pub const BUTTON_COLOR: Color = rgb(85, 165, 165);
pub const SEL_BUTTON_COLOR: Color = rgb(120, 190, 210);
pub const ACCENT_BUTTON_COLOR: Color = rgb(172, 188, 255);
pub const DARK_TEXT: Color = rgb(35, 42, 46);
pub const YELLOW: Color = rgb(223, 160, 0);

pub fn render(app: &mut App, f: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Max(TOOLBOX_WIDTH), Constraint::Min(0)],
    )
    .split(f.size());

    sidebar::render(app, f, main_layout[0]);
    canvas::render(app, f, main_layout[1]);

    match app.input_capture.mode {
        InputMode::Rename => rename::show(app, f, main_layout[1]),
        InputMode::Color => picker::show(app, f, main_layout[1]),
        InputMode::Help => help::show(f),
        InputMode::Normal => {}
    };
}
