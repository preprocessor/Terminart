use crate::app::{App, Result};
use crate::utils::cell::Cell;
use crate::utils::clicks::{ClickAction, Increment, LayerAction, SetValue};
use crate::utils::layer::Page;
use crate::TOOLBOX_WIDTH;
use ansi_style::{BGColor, Color as AColor};
use crossterm::event::MouseEventKind::{Down, Drag};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
use ratatui::style::Color;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match key_event.code {
        // Exit application on `ESC` or `Q`
        KeyCode::Esc | KeyCode::Char('Q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Reset
        KeyCode::Char('R') => app.reset(),
        // Brush size
        KeyCode::Char('s') => app.brush_up(1),
        KeyCode::Char('S') => app.brush_down(1),
        // Cycle foreground color through palette
        KeyCode::Char('f') => app.palette_next_fg(),
        KeyCode::Char('F') => app.palette_prev_fg(),
        // Cycle background color through palette
        KeyCode::Char('b') => app.palette_next_bg(),
        KeyCode::Char('B') => app.palette_prev_bg(),
        // Copy canvas contents to clipboard
        KeyCode::Char('Y') => copy_canvas_text(app)?,
        KeyCode::Char('y') => copy_canvas_ansi(app)?,
        // Use clipboard to set brush char
        KeyCode::Char('p') => clip_brush(app),
        // Help window
        KeyCode::Char('?') => app.toggle_help(),
        // Undo / Redo
        KeyCode::Char('u') => app.undo(),
        KeyCode::Char('U') => app.redo(),
        _ => {}
    }
    Ok(())
}

pub fn handle_mouse_events(event: MouseEvent, app: &mut App) -> Result<()> {
    let kind = event.kind;
    match kind {
        Down(btn) | Drag(btn) => {
            let x = event.column;
            let y = event.row;
            if let Some(action) = app.click_areas.get(&(x, y)).cloned() {
                // If the action isnt a draw action and the event is a drag event
                if action != ClickAction::Draw && kind == Drag(btn) {
                    // return early because we only want the canvas to respond to drag events
                    return Ok(());
                }
                let count = match event.modifiers {
                    KeyModifiers::CONTROL => 5,
                    KeyModifiers::ALT => 2,
                    _ => 1,
                };

                match action {
                    ClickAction::Draw => {
                        if kind == Down(btn) {
                            app.undo_history
                                .try_add_page(app.canvas.current_layer_name());
                        }

                        let x = x - TOOLBOX_WIDTH;

                        if kind == Down(MouseButton::Middle) {
                            paste_into_canvas(app, x, y)?;
                            return Ok(());
                        }

                        let size = app.brush.size;
                        let tool = app.brush.tool;

                        tool.draw(x, y, size, app);
                    }
                    ClickAction::Next(i) => match i {
                        Increment::CharPicker => app.char_picker.next(),
                        Increment::BrushSize => app.brush_up(count),
                    },
                    ClickAction::Prev(i) => match i {
                        Increment::CharPicker => app.char_picker.prev(),
                        Increment::BrushSize => app.brush_down(count),
                    },
                    ClickAction::Set(v) => match v {
                        SetValue::Tool(t) => app.brush.tool = t,
                        SetValue::Char(char) => app.brush.char = char,
                        SetValue::Color(color) => match btn {
                            MouseButton::Left => app.brush.fg = color,
                            MouseButton::Right => app.brush.bg = color,
                            MouseButton::Middle => match color {
                                c if c == app.brush.fg => app.brush.fg = Color::Reset,
                                c if c == app.brush.bg => app.brush.bg = Color::Reset,
                                _ => {}
                            },
                        },
                    },
                    ClickAction::Layer(action) => match action {
                        LayerAction::Add => app.canvas.add_layer(None),
                        LayerAction::Select(index) => app.canvas.select_layer(index),
                        LayerAction::Remove => app.remove_layer(),
                        LayerAction::Rename => todo!("Rename layer"),
                        LayerAction::MoveUp => app.canvas.move_layer_up(),
                        LayerAction::MoveDown => app.canvas.move_layer_down(),
                        LayerAction::ToggleVis => app.canvas.toggle_show(),
                    },
                    ClickAction::None => {}
                }
            }
        }

        _ => {}
    }
    Ok(())
}

fn clip_brush(app: &mut App) {
    if let Ok(s) = cli_clipboard::get_contents() {
        if let Some(c) = s.chars().next() {
            app.brush.char = c;
        }
    }
}

macro_rules! get_color {
    ($color:expr, $color_type:ty) => {
        match $color {
            Color::Reset => <$color_type>::Any,
            Color::Black => <$color_type>::Black,
            Color::Red => <$color_type>::Red,
            Color::Green => <$color_type>::Green,
            Color::Yellow => <$color_type>::Yellow,
            Color::Blue => <$color_type>::Blue,
            Color::Magenta => <$color_type>::Magenta,
            Color::Cyan => <$color_type>::Cyan,
            Color::Gray => <$color_type>::White,
            Color::DarkGray => <$color_type>::BlackBright,
            Color::LightRed => <$color_type>::RedBright,
            Color::LightGreen => <$color_type>::GreenBright,
            Color::LightYellow => <$color_type>::YellowBright,
            Color::LightBlue => <$color_type>::BlueBright,
            Color::LightMagenta => <$color_type>::MagentaBright,
            Color::LightCyan => <$color_type>::CyanBright,
            Color::White => <$color_type>::WhiteBright,
            Color::Rgb(r, g, b) => <$color_type>::RGB(r, g, b),
            Color::Indexed(i) => <$color_type>::Ansi256(i),
        }
    };
}

const fn get_ansi_colors(fg: Color, bg: Color) -> (AColor, BGColor) {
    let ansi_fg = get_color!(fg, AColor);
    let ansi_bg = get_color!(bg, BGColor);
    (ansi_fg, ansi_bg)
}

fn get_drawing_region(app: &App) -> Result<(u16, u16, u16, u16, Page)> {
    let (mut left, mut bottom) = (u16::MAX, u16::MAX);
    let (mut right, mut top) = (u16::MIN, u16::MIN);
    let page = app.canvas.render();
    for &(x, y) in page.keys() {
        left = left.min(x);
        right = right.max(x);
        bottom = bottom.min(y);
        top = top.max(y);
    }
    if left == u16::MAX || bottom == u16::MAX {
        return Err("No keys".into());
    }
    Ok((left, right, bottom, top, page))
}

fn copy_canvas_ansi(app: &App) -> Result<()> {
    let (left, right, bottom, top, page) = get_drawing_region(app)?;
    let mut lines_vec = Vec::with_capacity((top - bottom) as usize);

    for y in bottom..=top {
        let mut line = String::new();
        for x in left..=right {
            if let Some(cell) = page.get(&(x, y)) {
                let (fg, bg) = get_ansi_colors(cell.fg, cell.bg);

                line += &format!(
                    "{}{}{}{}{}",
                    fg.open(),
                    bg.open(),
                    cell.char,
                    fg.close(),
                    bg.close()
                );
            } else {
                line += " ";
            }
        }
        lines_vec.push(line);
    }

    let output_str = lines_vec.join("\n");

    if !output_str.is_empty() {
        cli_clipboard::set_contents(output_str)?;
    }

    Ok(())
}

fn copy_canvas_text(app: &App) -> Result<()> {
    let (left, right, bottom, top, page) = get_drawing_region(app)?;
    let mut lines_vec = Vec::with_capacity((top - bottom) as usize);

    for y in bottom..=top {
        let mut line = String::with_capacity((right - left) as usize);
        for x in left..=right {
            if let Some(cell) = page.get(&(x, y)) {
                line += &cell.char();
            } else {
                line += " ";
            }
        }
        lines_vec.push(line);
    }

    let output_str = lines_vec.join("\n");

    if !output_str.is_empty() {
        cli_clipboard::set_contents(output_str)?;
    }

    Ok(())
}

fn paste_into_canvas(app: &mut App, x: u16, y: u16) -> Result<()> {
    let (x, y) = (x as i16, y as i16);
    let clipboard = cli_clipboard::get_contents()?;
    for (dy, row) in clipboard.split('\n').enumerate() {
        for (dx, char) in row.chars().enumerate() {
            let (dx, dy) = (dx as i16, dy as i16);
            app.draw_cell(
                x + dx,
                y + dy,
                Cell {
                    char,
                    ..Default::default()
                },
            );
        }
    }
    Ok(())
}
