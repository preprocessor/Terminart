use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::ui::{BG, BG_DARK, LAYER_SELECTED, TOOL_BORDER, WHITE};
use crate::utils::clicks::LayerAction;
use crate::utils::{clicks::ClickAction, input::InputMode};

use super::Button;

pub fn render(app: &mut App, f: &mut Frame, area: Rect) {
    let block = outer_block(app, f, area);
    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Max(1), Constraint::Min(0)],
    )
    .split(block);

    render_buttons(app, f, layout[0]);
    render_layers(app, f, layout[1]);
}

fn outer_block(app: &mut App, f: &mut Frame, area: Rect) -> Rect {
    let block = Block::new()
        .title(Title::from(" Layers ".bold()).alignment(Alignment::Center))
        .title(Title::from(Button::accent("+")).alignment(Alignment::Right))
        .padding(Padding::horizontal(1))
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::new().fg(TOOL_BORDER));

    let add_layer_button = Rect {
        height: 1,
        width: 3,
        x: area.width - 3,
        ..area
    };
    app.input_capture.register_click(
        &add_layer_button,
        ClickAction::Layer(LayerAction::Add),
        InputMode::Normal,
    );

    let outer_block = block.inner(area);
    f.render_widget(block, area);

    outer_block
}

fn render_buttons(app: &mut App, f: &mut Frame, area: Rect) {
    let row = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Min(8),
            Constraint::Min(8),
            Constraint::Min(4),
            Constraint::Min(6),
        ],
    )
    .split(area);

    delete_button(app, f, row[0]);
    rename_button(app, f, row[1]);
    up_button(app, f, row[2]);
    down_button(app, f, row[3]);
}

fn render_layers(app: &mut App, f: &mut Frame, area: Rect) {
    let layers_count = app.canvas.layers.len();

    let mut constraints = vec![Constraint::Max(1); layers_count];
    constraints.push(Constraint::Min(0));

    let block = Block::new()
        .borders(Borders::TOP)
        .border_type(BorderType::QuadrantOutside)
        .border_style(Style::new().fg(BG).bg(BG_DARK));
    let block_inner = block.inner(area);

    f.render_widget(block, area);

    let rows = Layout::new(Direction::Vertical, constraints).split(block_inner);

    f.render_widget(Block::new().bg(BG_DARK), rows[layers_count]);

    for (i, (name, show)) in app.canvas.get_display_info() {
        let index = layers_count - (i + 1);
        let is_active_layer = index == app.canvas.active;

        // Selected layer background
        if is_active_layer {
            f.render_widget(Block::new().bg(LAYER_SELECTED).fg(WHITE), rows[i]);
        } else {
            f.render_widget(Block::new().bg(BG_DARK).fg(WHITE), rows[i]);
            app.input_capture.register_click(
                &rows[i],
                ClickAction::Layer(LayerAction::Select(index as u8)),
                InputMode::Normal,
            );
        }

        let row = Layout::new(
            Direction::Horizontal,
            [Constraint::Min(0), Constraint::Max(6)],
        )
        .split(rows[i]);

        // Layer
        f.render_widget(Paragraph::new(name), row[0]);

        // Visibility
        app.input_capture.register_click(
            &row[1],
            ClickAction::Layer(LayerAction::ToggleVis(index as u8)),
            InputMode::Normal,
        );
        let btn = if show {
            Button::normal("Hide")
        } else {
            Button::selected("Show")
        };
        f.render_widget(Paragraph::new(Line::from(btn)), row[1]);
    }
}

fn delete_button(app: &mut App, f: &mut Frame, area: Rect) {
    base_button(app, f, area, LayerAction::Remove, "Delete")
}

fn rename_button(app: &mut App, f: &mut Frame, area: Rect) {
    base_button(app, f, area, LayerAction::Rename, "Rename")
}

fn up_button(app: &mut App, f: &mut Frame, area: Rect) {
    base_button(app, f, area, LayerAction::MoveUp, "Up")
}

fn down_button(app: &mut App, f: &mut Frame, area: Rect) {
    base_button(app, f, area, LayerAction::MoveDown, "Down")
}

fn base_button(app: &mut App, f: &mut Frame, area: Rect, action: LayerAction, label: &str) {
    app.input_capture
        .register_click(&area, ClickAction::Layer(action), InputMode::Normal);
    f.render_widget(Paragraph::new(Line::from(Button::normal(label))), area);
}
