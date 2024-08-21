use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{widgets::Paragraph, Frame};

use crate::app::App;

pub fn show(app: &mut App, f: &mut Frame) {
    let terminal_area = f.area();

    let horiz_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .split(terminal_area);

    let undo_history = app
        .history
        .past
        .iter()
        .map(|i| format!("{:?}", i))
        .collect::<Vec<_>>()
        .join("\n");

    f.render_widget(
        Paragraph::new(format!("HISTORY\n\n{}", undo_history)),
        horiz_layout[0],
    );
}
