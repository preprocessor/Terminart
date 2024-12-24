use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{canvas::Canvas, Block, BorderType, Borders};
use ratatui::Frame;

use crate::{app::App, components::clicks::ClickAction, ui::DARK_TEXT};

pub fn render(app: &mut App, f: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .title(" Canvas ")
        .title_style(Style::new().bg(Color::Green).fg(DARK_TEXT));

    let block_inner = block.inner(area);
    app.input_capture
        .click_mode_normal(&block_inner, ClickAction::Draw);

    let width = block_inner.width as f64;
    let height = block_inner.height as f64;

    let render = app.layers.render();

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
