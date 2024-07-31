use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, BorderType, Borders},
    Frame,
};

use crate::{app::App, ui::DARK_TEXT, utils::clicks::ClickAction};

pub fn render(app: &mut App, f: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .title(" Canvas ")
        .title_style(Style::new().bg(Color::Green).fg(DARK_TEXT));

    let block_inner = block.inner(area);
    // app.input
    //     .register_click(&block_inner, ClickAction::Draw, InputMode::Normal);
    app.input_capture
        .click_mode_normal(&block_inner, ClickAction::Draw);

    let width = block_inner.width as f64;
    let height = block_inner.height as f64;

    let render = app.canvas.render();

    let canvas = Canvas::default()
        .x_bounds([0.0, width])
        .y_bounds([0.0, height])
        .paint(|c| {
            for (x, y, cell) in render
                .iter()
                .map(|(&(x, y), cell)| (x as f64, y as f64, cell))
            {
                c.print(x, height - y, Span::styled(cell.char(), cell.style()));
            }
        });

    f.render_widget(block, area);
    f.render_widget(canvas, block_inner);
}
