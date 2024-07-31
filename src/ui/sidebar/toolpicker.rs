use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph,
    },
    Frame,
};

use crate::{
    app::App,
    ui::{TOOL_BORDER, WHITE},
    utils::{
        clicks::{ClickAction, SetValue},
        input::InputMode,
        tools::Tool,
    },
};

use super::Button;

pub fn render(app: &mut App, f: &mut Frame, area: Rect) {
    let block_area = outer_block(f, area);
    let rows = Layout::new(Direction::Vertical, [Constraint::Min(1); 2]).split(block_area);
    render_buttons(app, f, rows[0]);
    render_info(app, f, rows[1]);
}

fn outer_block(f: &mut Frame, area: Rect) -> Rect {
    let block = Block::new()
        .title("Tool Selector ".bold())
        .title(
            Title::from(" ┈┈┄".bold())
                .position(Position::Bottom)
                .alignment(Alignment::Left),
        )
        .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(TOOL_BORDER));

    let block_inner = block.inner(area);

    f.render_widget(block, area);

    block_inner
}

fn render_buttons(app: &mut App, f: &mut Frame, area: Rect) {
    let current_tool = app.brush.tool;
    let tools = Tool::all();
    let tool_amount = tools.len();

    let row = Layout::new(Direction::Horizontal, vec![Constraint::Min(3); tool_amount]).split(area);

    tools.iter().zip(row.iter()).for_each(|(&t, &area)| {
        let c = t.char();

        let btn = if current_tool == t {
            Button::selected(&c)
        } else {
            Button::normal(&c)
        };

        let button = Paragraph::new(Line::from(btn));

        app.input_capture.register_click(
            &area,
            ClickAction::Set(SetValue::Tool(t)),
            InputMode::Normal,
        );
        f.render_widget(button, area);
    });
}

fn render_info(app: &App, f: &mut Frame, area: Rect) {
    let info = Paragraph::new(Line::from(vec![
        Span::from("Current tool: "),
        Span::from(app.brush.tool.name()).bold(),
    ]))
    .fg(WHITE)
    .alignment(Alignment::Center);

    f.render_widget(info, area);
}
