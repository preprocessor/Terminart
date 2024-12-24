use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::components::clicks::{ClickAction::Rename, PopupBoxAction};
use crate::components::save_load::FileSaveError;

use super::{centered_box, DARK_TEXT, WHITE};

pub fn show(app: &mut App, f: &mut Frame) {
    let area = f.area();
    let has_message = app.input_capture.text_area.error == Some(FileSaveError::NoName);
    let box_height = if has_message { 9 } else { 7 };
    let box_width = 38;

    let block_area = centered_box(box_width, box_height, area);

    app.input_capture
        .click_mode_popup(&block_area, Rename(PopupBoxAction::Nothing));

    let block = Block::new()
        .title(format!(
            " Rename layer: {} ",
            app.layers.get_active_layer().name
        ))
        .title_alignment(Alignment::Center)
        .title_style(Style::new().reversed().bold())
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    let block_inner = block.inner(block_area);

    f.render_widget(Clear, block_area);
    f.render_widget(block, block_area);

    let rows = Layout::new(
        Direction::Vertical,
        vec![Constraint::Min(1); box_height as usize - 2],
    )
    .split(block_inner);

    text(app, f, rows[1]);

    if has_message {
        message(f, rows[3]);
        buttons(app, f, rows[5]);
    } else {
        buttons(app, f, rows[3]);
    }
}

fn text(app: &App, f: &mut Frame, area: Rect) {
    let text_block_area = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length((area.width - 22) / 2),
            Constraint::Length(22),
            Constraint::Length((area.width - 22) / 2),
        ],
    )
    .split(area)[1];

    let text_block_bg = Block::new().bg(Color::DarkGray).fg(Color::White);

    let mut text_block_inner = text_block_bg.inner(text_block_area);
    text_block_inner.x += 1;
    text_block_inner.width -= 1;

    let display_text = Paragraph::new(app.input_capture.text_area.buffer.as_str());

    let cursor_area = Rect {
        x: text_block_inner.x + app.input_capture.text_area.pos as u16,
        width: 1,
        height: 1,
        ..text_block_inner
    };

    let cursor_block = Block::new().reversed();

    f.render_widget(cursor_block, cursor_area);
    f.render_widget(text_block_bg, text_block_area);
    f.render_widget(display_text, text_block_inner);
}

fn message(f: &mut Frame, area: Rect) {
    f.render_widget(
        Paragraph::new(Line::from(
            Span::from(" No layer name provided. ")
                .bg(Color::Red)
                .fg(WHITE),
        ))
        .alignment(Alignment::Center),
        area,
    )
}

fn buttons(app: &mut App, f: &mut Frame, area: Rect) {
    let buttons_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length(7),
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Length(6),
        ],
    )
    .split(area);

    let exit_area = buttons_layout[1];
    let accept_area = buttons_layout[3];

    let exit_button = Paragraph::new(" Cancel ")
        .alignment(Alignment::Center)
        .bold()
        .bg(Color::Red)
        .fg(DARK_TEXT);
    let accept_button = Paragraph::new(" Accept ")
        .alignment(Alignment::Center)
        .bold()
        .bg(Color::Blue)
        .fg(Color::White);

    app.input_capture
        .click_mode_popup(&exit_area, Rename(PopupBoxAction::Deny));
    f.render_widget(exit_button, exit_area);

    app.input_capture
        .click_mode_popup(&accept_area, Rename(PopupBoxAction::Accept));
    f.render_widget(accept_button, accept_area);
}
