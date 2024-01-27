use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::{
    app::App,
    utils::{
        clicks::{ClickAction, TypingAction},
        input::InputMode,
    },
    DARK_TEXT,
};

pub fn show(app: &mut App, f: &mut Frame, area: Rect) {
    let box_height = 7;
    let vert_center = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length((area.height - box_height) / 2),
            Constraint::Length(box_height),
            Constraint::Length((area.height - box_height) / 2),
        ],
    )
    .split(area)[1];

    let block_area = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Ratio(1, 6),
            Constraint::Ratio(2, 3),
            Constraint::Ratio(1, 6),
        ],
    )
    .split(vert_center)[1];

    app.input.register_click(
        &block_area,
        ClickAction::Typing(TypingAction::Nothing),
        InputMode::Rename,
    );

    let block = Block::new()
        .title(format!(
            " Rename layer: {} ",
            app.canvas.current_layer_name()
        ))
        .title_alignment(Alignment::Center)
        .title_style(Style::new().reversed())
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    let block_inner = block.inner(block_area);

    f.render_widget(Clear, block_area);
    f.render_widget(block, block_area);

    let rows = Layout::new(Direction::Vertical, [Constraint::Min(1); 5]).split(block_inner);

    text(app, f, rows[1]);
    buttons(app, f, rows[3]);
}

fn text(app: &App, f: &mut Frame, area: Rect) {
    let text_block_area = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Ratio(1, 6),
            Constraint::Ratio(2, 3),
            Constraint::Ratio(1, 6),
        ],
    )
    .split(area)[1];

    let text_block = Block::new().bg(Color::DarkGray).fg(Color::White);

    let text_block_inner = text_block.inner(text_block_area);

    let display_text = Paragraph::new(app.input.text.buffer.as_str());

    let cursor_area = Rect {
        x: text_block_inner.x + app.input.text.pos as u16,
        width: 1,
        height: 1,
        ..text_block_inner
    };

    let cursor_block = Block::new().reversed();

    f.render_widget(cursor_block, cursor_area);
    f.render_widget(text_block, text_block_area);
    f.render_widget(display_text, text_block_inner);
}

fn buttons(app: &mut App, f: &mut Frame, area: Rect) {
    let buttons_layout =
        Layout::new(Direction::Horizontal, [Constraint::Ratio(1, 5); 5]).split(area);

    let exit_area = buttons_layout[1];
    let accept_area = buttons_layout[3];

    let exit_button = Paragraph::new(" Cancel ")
        .alignment(Alignment::Center)
        .bg(Color::Red)
        .fg(DARK_TEXT);
    let accept_button = Paragraph::new(" Accept ")
        .alignment(Alignment::Center)
        .bg(Color::Blue)
        .fg(Color::White);

    app.input.register_click(
        &exit_area,
        ClickAction::Typing(TypingAction::Exit),
        InputMode::Rename,
    );
    f.render_widget(exit_button, exit_area);

    app.input.register_click(
        &accept_area,
        ClickAction::Typing(TypingAction::Accept),
        InputMode::Rename,
    );
    f.render_widget(accept_button, accept_area);
}
