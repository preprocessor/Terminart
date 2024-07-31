use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::ui::{ACCENT_BUTTON_COLOR, BG, BUTTON_COLOR, DARK_TEXT, SEL_BUTTON_COLOR, YELLOW};

use super::WHITE;

mod brushinfo;
mod charpicker;
mod colorpalette;
mod layermanager;
mod toolpicker;

pub fn render(app: &mut App, f: &mut Frame, area: Rect) {
    let bar_block = Block::new()
        .style(Style::new().bg(BG))
        .borders(Borders::all())
        .border_type(BorderType::QuadrantInside)
        .border_style(Style::new().fg(BG).bg(Color::Reset))
        .title(" Toolbox ".fg(DARK_TEXT).bg(YELLOW))
        .title_alignment(Alignment::Center);

    let bar_inner = bar_block.inner(area);

    f.render_widget(bar_block, area);

    let bar_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Max(3),  // 0: Brush info
            Constraint::Max(4),  // 1: Tools
            Constraint::Max(10), // 2: Char picker
            Constraint::Max(6),  // 3: Palette
            Constraint::Min(0),  // 4: Layers
            Constraint::Max(1),  // 5: Help text
        ],
    )
    .split(bar_inner);

    brushinfo::render(app, f, bar_layout[0]);
    toolpicker::render(app, f, bar_layout[1]);
    charpicker::render(app, f, bar_layout[2]);
    colorpalette::render(app, f, bar_layout[3]);
    layermanager::render(app, f, bar_layout[4]);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("Help: "),
            Span::raw("? ").bold(),
        ]))
        .fg(WHITE)
        .alignment(Alignment::Right),
        bar_layout[5],
    )
}

pub struct Button;
impl Button {
    pub fn custom(label: &str, bg: Color, fg: Color) -> Vec<Span> {
        vec![
            Span::raw("▐").fg(bg),
            Span::raw(label).bg(bg).fg(fg),
            Span::raw("▌").fg(bg),
        ]
    }

    pub fn blank(color: Color) -> Vec<Span<'static>> {
        vec![Span::raw("▐█▌").fg(color)]
    }

    pub fn normal(label: &str) -> Vec<Span> {
        Self::custom(label, BUTTON_COLOR, DARK_TEXT)
    }

    pub fn selected(label: &str) -> Vec<Span> {
        Self::custom(label, SEL_BUTTON_COLOR, DARK_TEXT)
    }

    pub fn accent(label: &str) -> Vec<Span> {
        Self::custom(label, ACCENT_BUTTON_COLOR, DARK_TEXT)
    }
}
