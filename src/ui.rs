use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, BorderType, Borders},
};

use crate::{app::App, ui_help as help, ui_sidebar as sidebar, utils::ClickAction, TOOLBOX_WIDTH};

/// Renders the user interface widgets.
pub fn render(app: &mut App, f: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Max(TOOLBOX_WIDTH), Constraint::Min(0)],
    )
    .split(f.size());

    sidebar::show(app, f, main_layout[0]);

    canvas(app, f, main_layout[1]);

    if app.needs_help {
        help::show(f);
    }
}

fn canvas(app: &mut App, f: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .title(" Canvas ")
        .title_style(Style::new().bg(Color::Green).fg(Color::Black));

    let block_inner = block.inner(area);
    app.register_click_area(&block_inner, ClickAction::Draw);

    let width = block_inner.width as f64;
    let height = block_inner.height as f64;

    let c = Canvas::default()
        .x_bounds([0.0, width])
        .y_bounds([0.0, height])
        .paint(|c| {
            for (x, y, cell) in app
                .drawing
                .iter()
                .map(|(&(x, y), cell)| (x as f64, y as f64, cell))
            {
                c.print(x, height - y, Span::styled(cell.char(), cell.style()));
            }
        });

    f.render_widget(block, area);
    f.render_widget(c, block_inner);
}
