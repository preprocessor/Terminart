use std::marker::PhantomData;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Padding, Paragraph,
    },
    Frame,
};

use crate::{
    app::App,
    utils::{ClickAction, Increment, SetValue, Tool},
    BLOCK, BUTTON_COLOR, BUTTON_COLOR_SEL, BUTTON_TEXT, BUTTON_TEXT_SEL, LOWER_BLOCK, TOOLBOX_BG,
    TOOL_BORDER, UPPER_BLOCK,
};

pub fn show(app: &mut App, f: &mut Frame, area: Rect) {
    let bar_block = Block::new()
        .style(Style::new().bg(TOOLBOX_BG))
        .borders(Borders::all())
        .border_type(BorderType::QuadrantInside)
        .border_style(Style::new().fg(TOOLBOX_BG).bg(Color::Reset))
        .title(" Toolbox ".fg(Color::Black).bg(Color::Yellow))
        .title_alignment(Alignment::Center);

    let bar_inner = bar_block.inner(area);

    f.render_widget(bar_block, area);

    let bar_layout = Layout::new(
        Direction::Vertical,
        [
            // TODO: Add layers and help text
            Constraint::Max(3),  // 0: Brush info
            Constraint::Max(4),  // 1: Tools
            Constraint::Max(10), // 2: Char picker
            Constraint::Max(6),  // 3: Palette
            Constraint::Min(0),  // 4: Layers
            Constraint::Max(1),  // 5: Help area
        ],
    )
    .split(bar_inner);

    BrushInfo::render(app, f, bar_layout[0]);
    ToolPicker::render(app, f, bar_layout[1]);
    CharPicker::render(app, f, bar_layout[2]);
    ColorPalette::render(app, f, bar_layout[3]);
}

fn make_button(l: &str) -> Vec<Span> {
    vec![
        Span::from("▐").fg(BUTTON_COLOR),
        Span::from(l).bg(BUTTON_COLOR).fg(BUTTON_TEXT),
        Span::from("▌").fg(BUTTON_COLOR),
    ]
}

fn make_button_sel(l: &str) -> Vec<Span> {
    vec![
        Span::from("▐").fg(BUTTON_COLOR_SEL),
        Span::from(l).bg(BUTTON_COLOR_SEL).fg(BUTTON_TEXT_SEL),
        Span::from("▌").fg(BUTTON_COLOR_SEL),
    ]
}

// ╭────────────╮
// │ Brush Info │
// ╰────────────╯
#[rustfmt::skip] struct BrushInfo<'a> { marker: PhantomData<&'a Frame<'a>> }
impl<'a> BrushInfo<'a> {
    fn render(app: &mut App, f: &mut Frame, area: Rect) {
        let block_area = Self::block(f, area);

        let brush_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Min(9), Constraint::Min(6), Constraint::Min(0)],
        )
        .split(block_area);

        Self::render_size_info(app, f, brush_layout[0]);
        Self::render_colors(app, f, brush_layout[1]);
        Self::render_char_info(app, f, brush_layout[2]);
    }

    fn block(f: &mut Frame, area: Rect) -> Rect {
        let brush_block = Block::new()
            .title("Brush ")
            .title_style(Style::new().bold())
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
        ]));

        f.render_widget(size_info, size_layout[0]);

        let size_button_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Min(3), Constraint::Min(1), Constraint::Min(3)],
        )
        .split(size_layout[1]);

        let size_down_area = size_button_layout[0];
        let size_up_area = size_button_layout[2];

        let size_down_button = Paragraph::new(Line::from(make_button("-")));
        let size_up_button = Paragraph::new(Line::from(make_button("+")));

        app.register_click_area(&size_down_area, ClickAction::Prev(Increment::BrushSize));
        f.render_widget(size_down_button, size_down_area);

        app.register_click_area(&size_up_area, ClickAction::Next(Increment::BrushSize));
        f.render_widget(size_up_button, size_up_area);
    }

    fn render_colors(app: &App, f: &mut Frame, area: Rect) {
        let current_colors = Paragraph::new(vec![
            Line::from(vec![
                Span::from("F").underlined(),
                Span::from("G:"),
                Span::from(BLOCK).fg(app.brush.fg),
            ]),
            Line::from(vec![
                Span::from("B").underlined(),
                Span::from("G:"),
                Span::from(BLOCK).fg(app.brush.bg),
            ]),
        ])
        .alignment(Alignment::Center);

        f.render_widget(current_colors, area);
    }

    fn render_char_info(app: &App, f: &mut Frame, area: Rect) {
        let brush = app.brush;
        let current_char = Paragraph::new(vec![
            Line::from(vec![Span::from("Character: "), Span::from(brush.char())]),
            Line::from(vec![
                Span::from("Preview: "),
                Span::styled(brush.char(), brush.style()),
            ]),
        ])
        .alignment(Alignment::Right);

        f.render_widget(current_char, area);
    }
}

// ╭─────────────╮
// │ Tool Picker │
// ╰─────────────╯
#[rustfmt::skip] struct ToolPicker<'a> { marker: PhantomData<&'a Frame<'a>> }
impl<'a> ToolPicker<'a> {
    fn render(app: &mut App, f: &mut Frame, area: Rect) {
        let block_area = Self::outer_block(f, area);
        let rows = Layout::new(Direction::Vertical, [Constraint::Min(1); 2]).split(block_area);
        Self::render_buttons(app, f, rows[0]);
        Self::render_info(app, f, rows[1]);
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

        let row =
            Layout::new(Direction::Horizontal, vec![Constraint::Min(3); tool_amount]).split(area);

        tools
            .iter()
            .zip(row.iter().take(tool_amount))
            .for_each(|(&t, &area)| {
                let c = t.char();

                let button = Paragraph::new(Line::from(if current_tool == t {
                    make_button_sel(&c)
                } else {
                    make_button(&c)
                }));

                app.register_click_area(&area, ClickAction::Set(SetValue::Tool(t)));
                f.render_widget(button, area);
            });
    }

    fn render_info(app: &App, f: &mut Frame, area: Rect) {
        let info = Paragraph::new(Line::from(vec![
            Span::from("Current tool: "),
            Span::from(app.brush.tool.name()).bold(),
        ]))
        .alignment(Alignment::Center);

        f.render_widget(info, area);
    }
}

// ╭──────────────────╮
// │ Character Picker │
// ╰──────────────────╯
#[rustfmt::skip] struct CharPicker<'a> { marker: PhantomData<&'a Frame<'a>> }
impl<'a> CharPicker<'a> {
    fn render(app: &mut App, f: &mut Frame, area: Rect) {
        let outer_block = Self::outer_block(app, f, area);
        let inner_block = Self::inner_block(app, f, outer_block);
        Self::render_buttons(app, f, inner_block);
    }

    fn outer_block(app: &mut App, f: &mut Frame, area: Rect) -> Rect {
        let block = Block::new()
            .title(Title::from(" Character Select ".bold()).alignment(Alignment::Center))
            .title(Title::from(make_button("<")).alignment(Alignment::Left))
            .title(Title::from(make_button(">")).alignment(Alignment::Right))
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
        app.register_click_area(&page_prev_button, ClickAction::Prev(Increment::CharPicker));
        app.register_click_area(&page_next_button, ClickAction::Next(Increment::CharPicker));

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
        let rows = Layout::new(Direction::Vertical, [Constraint::Min(2); 4]).split(area);
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

                let button = Paragraph::new(Line::from(if app.brush.char == c {
                    make_button_sel(&c_str)
                } else {
                    make_button(&c_str)
                }));

                app.register_click_area(&area, ClickAction::Set(SetValue::Char(c)));
                f.render_widget(button, area);
            });
    }
}

// ╭───────────────╮
// │ Color Palette │
// ╰───────────────╯
#[rustfmt::skip] struct ColorPalette<'a> { marker: PhantomData<&'a Frame<'a>> }
impl<'a> ColorPalette<'a> {
    fn render(app: &mut App, f: &mut Frame, area: Rect) {
        let block = Self::block(f, area);
        Self::render_buttons(app, f, block);
    }

    fn block(f: &mut Frame, area: Rect) -> Rect {
        let block = Block::new()
            .title(" Color palette ".bold())
            .title_alignment(Alignment::Center)
            .padding(Padding::horizontal(1))
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(TOOL_BORDER));

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
                let top_span = Span::raw(LOWER_BLOCK).fg(color);

                let bottom_style = match color {
                    c if c == app.brush.bg && c == app.brush.fg => {
                        Style::new().underlined().underline_color(Color::Magenta)
                    }
                    c if c == app.brush.fg => Style::new().underlined().underline_color(Color::Red),
                    c if c == app.brush.bg => {
                        Style::new().underlined().underline_color(Color::Blue)
                    }
                    _ => Style::new(),
                }
                .fg(color);

                let bottom_span = Span::raw(UPPER_BLOCK).style(bottom_style);

                let color_pg = Paragraph::new(vec![Line::from(top_span), Line::from(bottom_span)]);

                app.register_click_area(&area, ClickAction::Set(SetValue::Color(color)));
                f.render_widget(color_pg, area);
            });
    }
}