use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::components::clicks::ClickAction::{Next, Prev, Set};
use crate::components::clicks::Increment::BrushSize;
use crate::components::clicks::ResetValue::{BG, FG};
use crate::components::clicks::SetValue::Reset;

use super::{Button, DARK_TEXT, LIGHT_TEXT, TOOL_BORDER};

pub fn render(app: &mut App, f: &mut Frame, area: Rect) {
    let block_area = block(f, area);

    let brush_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length(9),
            Constraint::Length(6),
            Constraint::Min(0),
        ],
    )
    .split(block_area);

    render_size_info(app, f, brush_layout[0]);
    render_colors(app, f, brush_layout[1]);
    render_char_info(app, f, brush_layout[2]);
}

fn block(f: &mut Frame, area: Rect) -> Rect {
    let brush_block = Block::new()
        .title("Brush info ".bold())
        .borders(Borders::TOP)
        .border_style(Style::new().fg(TOOL_BORDER));

    let inner_block = brush_block.inner(area);

    f.render_widget(brush_block, area);

    inner_block
}

fn render_size_info(app: &mut App, f: &mut Frame, area: Rect) {
    let brush = app.brush;

    let size_layout = Layout::new(Direction::Vertical, [Constraint::Min(1); 2]).split(area);

    let size_info = Paragraph::new(Line::from(vec![
        Span::from("S").underlined(),
        Span::from("ize: "),
        Span::from(brush.size.to_string()),
    ]))
    .fg(LIGHT_TEXT);

    f.render_widget(size_info, size_layout[0]);

    let size_button_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
        ],
    )
    .split(size_layout[1]);

    let size_down_area = size_button_layout[0];
    let size_up_area = size_button_layout[2];

    let size_down_button = Paragraph::new(Line::from(Button::normal("-")));
    let size_up_button = Paragraph::new(Line::from(Button::normal("+")));

    app.input_capture
        .click_mode_normal(&size_down_area, Prev(BrushSize));
    f.render_widget(size_down_button, size_down_area);

    app.input_capture
        .click_mode_normal(&size_up_area, Next(BrushSize));
    f.render_widget(size_up_button, size_up_area);
}

fn render_colors(app: &mut App, f: &mut Frame, area: Rect) {
    let current_colors = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("▮").fg(DARK_TEXT),
            Span::raw("F").fg(LIGHT_TEXT).underlined(),
            Span::raw("G:").fg(LIGHT_TEXT),
            Span::from("██").fg(app.brush.fg),
        ]),
        Line::from(vec![
            Span::raw("▮").fg(LIGHT_TEXT),
            Span::raw("B").fg(LIGHT_TEXT).underlined(),
            Span::raw("G:").fg(LIGHT_TEXT),
            Span::raw("██").fg(app.brush.bg),
        ]),
    ])
    .alignment(Alignment::Center);

    let fg_area = Rect {
        height: 1,
        width: 3,
        x: area.x + 3,
        ..area
    };
    let bg_area = Rect {
        y: area.y + 1,
        ..fg_area
    };

    app.input_capture
        .click_mode_normal(&fg_area, Set(Reset(FG)));

    app.input_capture
        .click_mode_normal(&bg_area, Set(Reset(BG)));

    f.render_widget(current_colors, area);
}

fn render_char_info(app: &App, f: &mut Frame, area: Rect) {
    let brush = app.brush;
    let current_char = Paragraph::new(vec![
        Line::from(vec![Span::from("Character: "), Span::from(brush.char())]).fg(LIGHT_TEXT),
        Line::from(vec![
            Span::from("Preview: ").fg(LIGHT_TEXT),
            Span::styled(brush.char(), brush.style()),
        ]),
    ])
    .alignment(Alignment::Right);

    f.render_widget(current_char, area);
}
