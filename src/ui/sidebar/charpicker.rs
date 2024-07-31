use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{block::Title, Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::{
    app::App,
    ui::TOOL_BORDER,
    utils::{
        clicks::{ClickAction, Increment, SetValue},
        input::InputMode,
    },
};

use super::Button;

pub fn render(app: &mut App, f: &mut Frame, area: Rect) {
    let outer_block = outer_block(app, f, area);
    let inner_block = inner_block(app, f, outer_block);
    render_buttons(app, f, inner_block);
}

fn outer_block(app: &mut App, f: &mut Frame, area: Rect) -> Rect {
    let block = Block::new()
        .title(Title::from(" Character Select ".bold()).alignment(Alignment::Center))
        .title(Title::from(Button::accent("<")).alignment(Alignment::Left))
        .title(Title::from(Button::accent(">")).alignment(Alignment::Right))
        .padding(Padding::horizontal(1))
        .borders(Borders::TOP)
        .border_style(Style::new().fg(TOOL_BORDER));

    let page_prev_button = Rect {
        height: 1,
        width: 3,
        ..area
    };

    let page_next_button = Rect {
        x: area.width - 3,
        ..page_prev_button
    };
    app.input_capture.register_click(
        &page_prev_button,
        ClickAction::Prev(Increment::CharPicker),
        InputMode::Normal,
    );
    app.input_capture.register_click(
        &page_next_button,
        ClickAction::Next(Increment::CharPicker),
        InputMode::Normal,
    );

    let outer_block = block.inner(area);
    f.render_widget(block, area);

    outer_block
}

fn inner_block(app: &App, f: &mut Frame, area: Rect) -> Rect {
    let char_block = Block::new()
        .title(Title::from(vec![
            Span::from((app.char_picker.page + 1).to_string()),
            Span::from("/"),
            Span::from((app.char_picker.rows() + 1).to_string()),
        ]))
        .borders(Borders::all())
        .border_type(BorderType::Double)
        .border_style(Style::new().fg(TOOL_BORDER));

    let inner_block = char_block.inner(area);
    f.render_widget(char_block, area);

    inner_block
}

fn render_buttons(app: &mut App, f: &mut Frame, area: Rect) {
    let rows = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(2),
            Constraint::Min(2),
            Constraint::Min(2),
            Constraint::Min(1),
        ],
    )
    .split(area);
    let row = Layout::new(Direction::Horizontal, [Constraint::Min(3); 8]);

    let row1 = row.split(rows[0]);
    let row2 = row.split(rows[1]);
    let row3 = row.split(rows[2]);
    let row4 = row.split(rows[3]);

    let row_iter = row1
        .iter()
        .chain(row2.iter())
        .chain(row3.iter())
        .chain(row4.iter());

    app.char_picker
        .page()
        .iter()
        .zip(row_iter)
        .for_each(|(&c, &area)| {
            // replace space with a nicer character
            let c_str = if c == ' ' {
                "␣".to_string()
            } else {
                c.to_string()
            };

            let btn = if app.brush.char == c {
                Button::selected(&c_str)
            } else {
                Button::normal(&c_str)
            };

            let button = Paragraph::new(Line::from(btn));

            app.input_capture.register_click(
                &area,
                ClickAction::Set(SetValue::Char(c)),
                InputMode::Normal,
            );
            f.render_widget(button, area);
        });
}
