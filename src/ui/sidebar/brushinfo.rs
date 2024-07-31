use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::ui::{BLACK, TOOL_BORDER, WHITE};
use crate::utils::clicks::{ClickAction, Increment, ResetValue, SetValue};
use crate::utils::input::InputMode;

use super::Button;

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
    .fg(WHITE);

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

    app.input_capture.register_click(
        &size_down_area,
        ClickAction::Prev(Increment::BrushSize),
        InputMode::Normal,
    );
    f.render_widget(size_down_button, size_down_area);

    app.input_capture.register_click(
        &size_up_area,
        ClickAction::Next(Increment::BrushSize),
        InputMode::Normal,
    );
    f.render_widget(size_up_button, size_up_area);
}

fn render_colors(app: &mut App, f: &mut Frame, area: Rect) {
    let current_colors = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("▮").fg(BLACK),
            Span::raw("F").fg(WHITE).underlined(),
            Span::raw("G:").fg(WHITE),
            Span::from("██").fg(app.brush.fg),
        ]),
        Line::from(vec![
            Span::raw("▮").fg(WHITE),
            Span::raw("B").fg(WHITE).underlined(),
            Span::raw("G:").fg(WHITE),
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

    app.input_capture.register_click(
        &fg_area,
        ClickAction::Set(SetValue::Reset(ResetValue::FG)),
        InputMode::Normal,
    );

    app.input_capture.register_click(
        &bg_area,
        ClickAction::Set(SetValue::Reset(ResetValue::BG)),
        InputMode::Normal,
    );

    f.render_widget(current_colors, area);
}

fn render_char_info(app: &App, f: &mut Frame, area: Rect) {
    let brush = app.brush;
    let current_char = Paragraph::new(vec![
        Line::from(vec![Span::from("Character: "), Span::from(brush.char())]).fg(WHITE),
        Line::from(vec![
            Span::from("Preview: ").fg(WHITE),
            Span::styled(brush.char(), brush.style()),
        ]),
    ])
    .alignment(Alignment::Right);

    f.render_widget(current_char, area);
}
