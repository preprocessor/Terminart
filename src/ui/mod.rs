use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::utils::input::InputFocus;
use crate::{app::App, TOOLBOX_WIDTH};

mod canvas;
pub mod help;
pub mod rename;
pub mod sidebar;

pub fn render(app: &mut App, f: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Max(TOOLBOX_WIDTH), Constraint::Min(0)],
    )
    .split(f.size());

    sidebar::render(app, f, main_layout[0]);
    canvas::render(app, f, main_layout[1]);

    match app.input.mode {
        InputFocus::Rename => rename::show(app, f, main_layout[1]),
        InputFocus::Color => todo!("Color picker popup"),
        InputFocus::Help => help::show(f),
        InputFocus::Normal => {}
    };
}
