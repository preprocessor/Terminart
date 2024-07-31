use std::rc::Rc;

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

use super::{BG, BLACK, BUTTON_COLOR, COLOR_STEP, COLOR_STEPS, DARK_TEXT, TOOL_BORDER};
use crate::app::App;
use crate::utils::clicks::PickAction;
use crate::utils::input::color::TextFocus;

use super::sidebar::Button;

pub fn show(app: &mut App, f: &mut Frame, area: Rect) {
    let box_height = 16;
    let box_width = 66;

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
            Constraint::Length((area.width - box_width) / 2),
            Constraint::Length(box_width),
            Constraint::Length((area.width - box_width) / 2),
        ],
    )
    .split(vert_center)[1];

    app.input_capture
        .click_mode_colorpicker(&block_area, PickAction::Nothing);

    let block = Block::new()
        .title(" Color Picker ")
        .title_alignment(Alignment::Center)
        .title_style(Style::new().reversed())
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    let block_inner = block.inner(block_area);

    f.render_widget(Clear, block_area);
    f.render_widget(block, block_area);

    let horiz_split = Layout::new(
        Direction::Vertical,
        [Constraint::Min(9), Constraint::Min(7)],
    )
    .split(block_inner);

    let top_half = horiz_split[0];
    let bot_half = horiz_split[1];

    let column_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Min(1),
            Constraint::Min(9),
            Constraint::Min(1),
            Constraint::Min(COLOR_STEPS as u16),
            Constraint::Min(1),
            Constraint::Min(18),
        ],
    );

    let cols = column_layout.split(top_half);

    let values_col = cols[1];
    let sliders_col = cols[3];
    let info_col = cols[5];

    rgb_boxes_and_buttons(app, f, values_col);
    sliders(app, f, sliders_col);
    preview_block(app, f, info_col);
    control_buttons(app, f, column_layout.split(bot_half));
}

fn rgb_boxes_and_buttons(app: &mut App, f: &mut Frame, area: Rect) {
    let rows = Layout::new(Direction::Vertical, [Constraint::Min(3); 3]).split(area);

    let color_values = app.input_capture.color_picker.colors();
    let text_focus = app.input_capture.color_picker.focus;

    let base_row = Layout::new(Direction::Vertical, [Constraint::Min(1); 3]);

    for (&row, (value, name)) in rows.iter().zip(color_values.into_iter()) {
        let layout = base_row.split(row);

        let title = Paragraph::new(format!("{:?}", name)).alignment(Alignment::Center);

        f.render_widget(title, layout[0]);

        let text_bg = Block::new().bg(BG).fg(TOOL_BORDER);

        let controls_layout =
            Layout::new(Direction::Horizontal, [Constraint::Min(3); 3]).split(layout[1]);

        let minus_area = controls_layout[0];
        let text_bg_area = controls_layout[1];
        let plus_area = controls_layout[2];

        let text_area = text_bg.inner(text_bg_area);

        app.input_capture
            .click_mode_colorpicker(&text_bg_area, PickAction::ChangeFocus(name));
        f.render_widget(text_bg, text_bg_area);

        let text = Paragraph::new(value.to_string());

        f.render_widget(text, text_area);

        // If the box is focused
        if name == text_focus {
            let cursor_area = Rect {
                x: text_area.x + app.input_capture.color_picker.pos(),
                width: 1,
                height: 1,
                ..text_area
            };

            let cursor_block = Block::new().reversed();

            f.render_widget(cursor_block, cursor_area);
        }

        let minus_button = Paragraph::new(Line::from(Button::normal("-")));
        let plus_button = Paragraph::new(Line::from(Button::normal("+")));

        app.input_capture
            .click_mode_colorpicker(&minus_area, PickAction::Minus(name));
        f.render_widget(minus_button, minus_area);

        app.input_capture
            .click_mode_colorpicker(&plus_area, PickAction::Plus(name));
        f.render_widget(plus_button, plus_area);
    }
}

fn sliders(app: &mut App, f: &mut Frame, area: Rect) {
    let rows = Layout::new(Direction::Vertical, [Constraint::Min(3); 3]).split(area);

    let color_values = app.input_capture.color_picker.colors();

    let base_layout = Layout::new(Direction::Vertical, [Constraint::Min(1); 3]);
    let column_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Min(1); COLOR_STEPS as usize],
    );

    for (&row, (color_value, color_name)) in rows.iter().zip(color_values.into_iter()) {
        let base = base_layout.split(row);

        let upper_row = column_layout.split(base[0]);
        let colors_row = column_layout.split(base[1]);
        let lower_row = column_layout.split(base[2]);

        let active_column = color_value.div_ceil(COLOR_STEP) as usize;

        f.render_widget(Paragraph::new("┬"), upper_row[active_column]);
        f.render_widget(Paragraph::new("┴"), lower_row[active_column]);

        for i in 0..COLOR_STEPS as usize {
            let color_strength = COLOR_STEP.saturating_mul(i as u8);

            let row_color = match color_name {
                TextFocus::Hex => Color::White, // This won't be used
                TextFocus::Red => Color::Rgb(color_strength, 0, 0),
                TextFocus::Green => Color::Rgb(0, color_strength, 0),
                TextFocus::Blue => Color::Rgb(0, 0, color_strength),
            };

            if i == active_column {
                f.render_widget(Paragraph::new("│").bg(row_color), colors_row[i]);
                continue;
            }

            app.input_capture.click_mode_colorpicker(
                &colors_row[i],
                PickAction::Update(color_name, color_strength),
            );
            f.render_widget(Paragraph::new(" ").bg(row_color), colors_row[i]);
        }
    }
}

fn preview_block(app: &mut App, f: &mut Frame, area: Rect) {
    let center = Layout::new(
        Direction::Horizontal,
        [Constraint::Min(2), Constraint::Min(14), Constraint::Min(2)],
    )
    .split(area)[1];

    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(1),
            Constraint::Min(7),
            Constraint::Min(1),
            Constraint::Min(1),
        ],
    )
    .split(center);

    let current_color = app.input_capture.color_picker.get_style_color();

    let preview = Block::new().bg(current_color);

    f.render_widget(preview, layout[1]);
}

fn control_buttons(app: &mut App, f: &mut Frame, areas: Rc<[Rect]>) {
    let left = areas[1];
    let center = areas[3];
    let right = areas[5];

    hex_input_and_exit(app, f, left);
    replace_palette_color(app, f, center);
    set_brush_colors(app, f, right);
}

fn hex_input_and_exit(app: &mut App, f: &mut Frame, area: Rect) {
    let layout = Layout::new(
        Direction::Vertical,
        vec![Constraint::Min(1); area.height as usize],
    )
    .split(area);

    let title = Paragraph::new("Hex").alignment(Alignment::Center);
    f.render_widget(title, layout[0]);

    let text_bg = Block::new().bg(BG).fg(TOOL_BORDER);

    let text_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Min(1), // Space
            Constraint::Min(1), // #
            Constraint::Min(6), // Hex
            Constraint::Min(1), // Space
        ],
    )
    .split(layout[1]);

    let text_bg_area = Rect {
        width: 7,
        ..text_layout[1]
    }; // Combine the 2 inner areas
    let text_area = text_layout[2];

    let text = Paragraph::new(app.input_capture.color_picker.get_hex_str());

    f.render_widget(Paragraph::new("▐").fg(BG), text_layout[0]);
    f.render_widget(Paragraph::new("#"), text_layout[1]);
    f.render_widget(text, text_area);
    f.render_widget(Paragraph::new("▌").fg(BG), text_layout[3]);

    app.input_capture
        .click_mode_colorpicker(&text_bg_area, PickAction::ChangeFocus(TextFocus::Hex));
    f.render_widget(text_bg, text_bg_area);

    let text_focus = app.input_capture.color_picker.focus;
    // If the box is focused
    if text_focus == TextFocus::Hex {
        let cursor_area = Rect {
            x: text_area.x + app.input_capture.color_picker.pos(),
            width: 1,
            height: 1,
            ..text_area
        };

        let cursor_block = Block::new().reversed();

        f.render_widget(cursor_block, cursor_area);
    }

    let exit_button = Paragraph::new(Line::from(Button::custom(
        "Close",
        Color::Rgb(180, 30, 30),
        TOOL_BORDER,
    )))
    .alignment(Alignment::Center);

    app.input_capture
        .click_mode_colorpicker(&layout[3], PickAction::Exit);

    f.render_widget(exit_button, layout[3]);
}

fn replace_palette_color(app: &mut App, f: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::all())
        .title(Title::from(" Replace palette color ".bold()).alignment(Alignment::Center))
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(TOOL_BORDER));

    let block_center = Layout::new(
        Direction::Horizontal,
        [Constraint::Min(6), Constraint::Max(26), Constraint::Min(0)],
    )
    .split(area)[1];

    let block_inner = block.inner(block_center);
    f.render_widget(block, block_center);

    let rows = Layout::new(Direction::Vertical, [Constraint::Min(1); 3]).split(block_inner);

    let cols_layout = Layout::new(Direction::Horizontal, [Constraint::Min(3); 8]);

    let row1 = cols_layout.split(rows[0]);
    let row2 = cols_layout.split(rows[2]);

    let row_iter = row1.iter().chain(row2.iter());

    app.palette
        .colors()
        .iter()
        .zip(row_iter)
        .enumerate()
        .for_each(|(i, (&color, &area))| {
            let button = Paragraph::new(Line::from(Button::blank(color)));

            app.input_capture.click_mode_colorpicker(
                &area,
                PickAction::ReplacePColor(app.input_capture.color_picker.get_style_color(), i),
            );

            f.render_widget(button, area);
        });
}

fn set_brush_colors(app: &mut App, f: &mut Frame, area: Rect) {
    let layout = Layout::new(
        Direction::Vertical,
        vec![Constraint::Min(1); area.height as usize],
    )
    .split(area);

    let fg_button = Paragraph::new(Line::from(vec![
        Span::raw("▐").fg(BUTTON_COLOR),
        Span::raw("■").bg(BUTTON_COLOR).fg(BLACK),
        Span::raw(" Set to FG").bg(BUTTON_COLOR).fg(DARK_TEXT),
        Span::raw("▌").fg(BUTTON_COLOR),
    ]))
    .alignment(Alignment::Center);

    let bg_button = Paragraph::new(Line::from(vec![
        Span::raw("▐").fg(BUTTON_COLOR),
        Span::raw("■").bg(BUTTON_COLOR).fg(BLACK),
        Span::raw(" Set to BG").bg(BUTTON_COLOR).fg(DARK_TEXT),
        Span::raw("▌").fg(BUTTON_COLOR),
    ]))
    .alignment(Alignment::Center);

    app.input_capture
        .click_mode_colorpicker(&layout[1], PickAction::AcceptFG);
    app.input_capture
        .click_mode_colorpicker(&layout[3], PickAction::AcceptBG);

    f.render_widget(fg_button, layout[1]);
    f.render_widget(bg_button, layout[3]);
}
