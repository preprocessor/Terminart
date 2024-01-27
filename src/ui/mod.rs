// TODO: ADD STATEFUL WIDGETS
mod canvas;
pub mod help;
pub mod rename;
pub mod sidebar;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::utils::input::InputMode;
use crate::{app::App, TOOLBOX_WIDTH};

pub fn render(app: &mut App, f: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Max(TOOLBOX_WIDTH), Constraint::Min(0)],
    )
    .split(f.size());

    sidebar::render(app, f, main_layout[0]);
    canvas::render(app, f, main_layout[1]);

    match app.input.mode {
        InputMode::Rename => rename::show(app, f, main_layout[1]),
        InputMode::Color(_color_field) => todo!("Color picker popup"),
        InputMode::Help => help::show(f),
        InputMode::Normal => {}
    };
}
