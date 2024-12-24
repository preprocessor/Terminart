use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{block::Title, Block, BorderType, Borders, Padding, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::components::clicks::ClickAction::{PickColor, Set};
use crate::components::clicks::PickAction::New;
use crate::components::clicks::SetValue::Color;

use super::{Button, BG_LAYER_MANAGER, BLACK, TOOL_BORDER, WHITE};

pub fn render(app: &mut App, f: &mut Frame, area: Rect) {
    let block = block(app, f, area);
    render_buttons(app, f, block);
}

fn block(app: &mut App, f: &mut Frame, area: Rect) -> Rect {
    let block = Block::new()
        .title(Title::from(" Palette ".bold()).alignment(Alignment::Center))
        .title(Title::from(Button::accent("+")).alignment(Alignment::Right))
        .padding(Padding::horizontal(1))
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(TOOL_BORDER));

    let add_color_button = Rect {
        height: 1,
        width: 3,
        x: area.width - 3,
        ..area
    };
    app.input_capture
        .click_mode_normal(&add_color_button, PickColor(New));

    let block_inner = block.inner(area);
    f.render_widget(block, area);

    block_inner
}

fn render_buttons(app: &mut App, f: &mut Frame, area: Rect) {
    let rows = Layout::new(Direction::Vertical, [Constraint::Min(2); 2]).split(area);
    let row = Layout::new(Direction::Horizontal, [Constraint::Min(3); 8]);
    let row1 = row.split(rows[0]);
    let row2 = row.split(rows[1]);

    let row_iter = row1.iter().chain(row2.iter());

    app.palette
        .colors()
        .iter()
        .zip(row_iter)
        .for_each(|(&color, &area)| {
            let style = match color {
                c if c == app.brush.bg && c == app.brush.fg => Style::new().bg(BG_LAYER_MANAGER),
                c if c == app.brush.fg => Style::new().bg(WHITE),
                c if c == app.brush.bg => Style::new().bg(BLACK),
                _ => Style::new(),
            }
            .fg(color);

            let top_line = Line::from(vec![
                Span::raw("▗").fg(color),
                Span::raw("▄").style(style),
                Span::raw("▖").fg(color),
            ]); // ▗▄▖

            let bot_line = Line::from(Span::raw("▝▀▘").fg(color));

            let color_pg = Paragraph::new(vec![top_line, bot_line]);

            app.input_capture
                .click_mode_normal(&area, Set(Color(color)));
            f.render_widget(color_pg, area);
        });
}
