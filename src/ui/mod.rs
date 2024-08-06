mod canvas;
mod popup_colorpicker;
mod popup_exit_confirm;
mod popup_export;
mod popup_help;
mod popup_rename;
mod popup_save;
mod sidebar;
mod too_small;

pub const TOOLBOX_WIDTH: u16 = 30;
pub const _COLOR_STEPS: u8 = 32;
pub const COLOR_STEPS: u8 = _COLOR_STEPS + 1;
pub const COLOR_STEP_AMT: u8 = u8::MAX / COLOR_STEPS + 1;

pub const WHITE: Color = rgb(220, 220, 220);
pub const LIGHT_TEXT: Color = WHITE;
pub const DIM_TEXT: Color = rgb(170, 170, 170);
pub const BLACK: Color = rgb(10, 10, 10);
pub const BG: Color = rgb(100, 100, 100);
pub const BG_LAYER_MANAGER: Color = rgb(70, 70, 70);
pub const LAYER_SELECTED: Color = rgb(120, 120, 120);
pub const LAYER_UNSELECTED: Color = rgb(90, 90, 90);
pub const TOOL_BORDER: Color = rgb(240, 240, 240);
pub const BUTTON_COLOR: Color = rgb(85, 165, 165);
pub const SEL_BUTTON_COLOR: Color = rgb(120, 190, 210);
pub const ACCENT_BUTTON_COLOR: Color = rgb(172, 188, 255);
pub const DARK_TEXT: Color = rgb(35, 42, 46);
pub const YELLOW: Color = rgb(223, 160, 0);

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color::{self, Rgb as rgb}; //  make the struct work with rgb highlighter
use ratatui::Frame;

use crate::app::App;
use crate::components::input::InputMode;

pub fn render(app: &mut App, f: &mut Frame) {
    let terminal_size = f.size();

    if terminal_size.width < 70 || terminal_size.height < 30 {
        app.input_capture.change_mode(InputMode::TooSmall);
        too_small::show(f);
        return;
    }

    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Length(TOOLBOX_WIDTH), Constraint::Min(0)],
    )
    .split(terminal_size);

    sidebar::render(app, f, main_layout[0]);
    canvas::render(app, f, main_layout[1]);

    match app.input_capture.mode {
        InputMode::Rename => popup_rename::show(app, f),
        InputMode::Color => popup_colorpicker::show(app, f),
        InputMode::Help => popup_help::show(f),
        InputMode::Export => popup_export::show(app, f),
        InputMode::Save => popup_save::show(app, f),
        InputMode::Exit => popup_exit_confirm::show(app, f),
        _ => {}
    };
}

pub fn centered_box(width: u16, height: u16, area: Rect) -> Rect {
    let vert_center = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length((area.height - height) / 2),
            Constraint::Length(height),
            Constraint::Length((area.height - height) / 2),
        ],
    )
    .split(area)[1];

    Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length((area.width - width) / 2),
            Constraint::Length(width),
            Constraint::Length((area.width - width) / 2),
        ],
    )
    .split(vert_center)[1]
}
