use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::components::clicks::{ClickAction::Exit, PopupBoxAction::*};

use super::centered_box;
use super::DARK_TEXT;

pub fn show(app: &mut App, f: &mut Frame) {
    let area = f.area();
    let box_height = 7;
    let box_width = 19;

    let block_area = centered_box(box_width, box_height, area);

    app.input_capture
        .click_mode_popup(&block_area, Exit(Nothing));

    let block = Block::new()
        .title(" Exit ")
        .title_alignment(Alignment::Center)
        .title_style(Style::new().reversed().bold())
        .borders(Borders::all())
        .padding(Padding::new(1, 1, 1, 1))
        .border_type(BorderType::Rounded);

    let block_inner = block.inner(block_area);

    f.render_widget(Clear, block_area);
    f.render_widget(block, block_area);

    let rows = Layout::new(Direction::Vertical, [Constraint::Length(1); 3]).split(block_inner);

    f.render_widget(
        Paragraph::new("Are you sure?")
            .alignment(Alignment::Center)
            .bold(),
        rows[0],
    );

    buttons(app, f, rows[2]);
}

fn buttons(app: &mut App, f: &mut Frame, area: Rect) {
    let buttons_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length(2),
            Constraint::Length(5),
            Constraint::Length(2),
            Constraint::Length(4),
            Constraint::Length(2),
        ],
    )
    .split(area);

    let exit_area = buttons_layout[1];
    let stay_area = buttons_layout[3];

    let exit_button = Paragraph::new(Line::from(vec![
        Span::from(" "),
        Span::from("Y").underlined(),
        Span::from("es "),
    ]))
    .alignment(Alignment::Center)
    .bold()
    .bg(Color::Red)
    .fg(DARK_TEXT);

    let stay_button = Paragraph::new(Line::from(vec![
        Span::from(" "),
        Span::from("N").underlined(),
        Span::from("o "),
    ]))
    .alignment(Alignment::Center)
    .bold()
    .bg(Color::Blue)
    .fg(Color::White);

    app.input_capture.click_mode_popup(&exit_area, Exit(Accept));
    f.render_widget(exit_button, exit_area);

    app.input_capture.click_mode_popup(&stay_area, Exit(Deny));
    f.render_widget(stay_button, stay_area);
}
