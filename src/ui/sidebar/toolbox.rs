use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::components::clicks::{ClickAction::Set, SetValue::Tool};
use crate::components::tools::Tools;

use super::{Button, LIGHT_TEXT, TOOL_BORDER};

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
            Title::from(" ╶".bold())
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
    let tools = Tools::all();
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

        app.input_capture.click_mode_normal(&area, Set(Tool(t)));
        f.render_widget(button, area);
    });
}

fn render_info(app: &App, f: &mut Frame, area: Rect) {
    let info = Paragraph::new(Line::from(vec![
        Span::from("Current tool: "),
        Span::from(app.brush.tool.to_string()).bold(),
    ]))
    .fg(LIGHT_TEXT)
    .alignment(Alignment::Center);

    f.render_widget(info, area);
}
