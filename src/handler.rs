use crate::app::{App, Result};
use crate::utils::cell::Cell;
use crate::utils::clicks::{ClickAction, Increment, LayerAction, SetValue, TypingAction};
use crate::utils::input::InputFocus;
use crate::utils::layer::Page;
use crate::TOOLBOX_WIDTH;
use ansi_style::{BGColor, Color as AColor};
use crossterm::event::MouseEventKind::{Down, Drag, Up};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
use ratatui::style::Color;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match app.input.mode {
        InputFocus::Normal => {
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
                KeyCode::Char('s') => app.brush.up(1),
                KeyCode::Char('S') => app.brush.down(1),
                // Cycle foreground color through palette
                KeyCode::Char('f') => app.brush_next_fg(),
                KeyCode::Char('F') => app.brush_prev_fg(),
                // Cycle background color through palette
                KeyCode::Char('b') => app.brush_next_bg(),
                KeyCode::Char('B') => app.brush_prev_bg(),
                // Copy canvas contents to clipboard
                KeyCode::Char('Y') => copy_canvas_text(app)?,
                KeyCode::Char('y') => copy_canvas_ansi(app)?,
                // Use clipboard to set brush char
                KeyCode::Char('p') => clip_brush(app),
                // Help window
                KeyCode::Char('?') => app.input.toggle_help(),
                // Undo / Redo
                KeyCode::Char('u') => app.undo(),
                KeyCode::Char('U') => app.redo(),
                _ => {}
            }
        }
        _ => match key_event.code {
            KeyCode::Esc => {
                app.input.mode = InputFocus::Normal;
            }
            KeyCode::Char('c' | 'C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                } else {
                    todo!("pass c through")
                }
            }
            KeyCode::Char(_char) => todo!(),
            KeyCode::Backspace => todo!(),
            KeyCode::Left => todo!(),
            KeyCode::Right => todo!(),
            KeyCode::Home => todo!(),
            KeyCode::End => todo!(),
            KeyCode::Enter => todo!(),
            _ => {}
        },
    }
    Ok(())
}

pub fn handle_mouse_events(event: MouseEvent, app: &mut App) -> Result<()> {
    let kind = event.kind;
    let x = event.column;
    let y = event.row;

    match app.input.mode {
        InputFocus::Normal => {
            match kind {
                Up(_) => {
                    // if event.modifiers != KeyModifiers::CONTROL {
                    app.canvas.last_pos = None
                    // }
                }
                Down(btn) => {
                    if let Some(&action) = app.input.get(x, y) {
                        // If the action isnt a draw action and the event is a drag event
                        let count = match event.modifiers {
                            KeyModifiers::CONTROL => 5,
                            KeyModifiers::ALT => 2,
                            _ => 1,
                        };

                        match action {
                            ClickAction::Draw => {
                                app.undo_history
                                    .try_add_page(app.canvas.current_layer_name());

                                if btn == MouseButton::Middle {
                                    paste_into_canvas(app, x - TOOLBOX_WIDTH, y)?;
                                    return Ok(());
                                }

                                draw(x, y, app);
                            }
                            ClickAction::Next(i) => match i {
                                Increment::CharPicker => app.char_picker.next(),
                                Increment::BrushSize => app.brush.up(count),
                            },
                            ClickAction::Prev(i) => match i {
                                Increment::CharPicker => app.char_picker.prev(),
                                Increment::BrushSize => app.brush.down(count),
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
                                LayerAction::Rename => {
                                    app.input.mode = InputFocus::Rename;
                                    // todo!("Rename layer")
                                }
                                LayerAction::MoveUp => app.canvas.move_layer_up(),
                                LayerAction::MoveDown => app.canvas.move_layer_down(),
                                LayerAction::ToggleVis(index) => app.canvas.toggle_show(index),
                            },
                            ClickAction::Typing(_) => {}
                        }
                    }
                }

                Drag(MouseButton::Left | MouseButton::Right) => {
                    if let Some(&action) = app.input.get(x, y) {
                        // If the action isnt a draw action
                        if action != ClickAction::Draw {
                            // return early because we only want the canvas to respond to drag events
                            return Ok(());
                        }

                        draw(x, y, app);
                    }
                }
                _ => {}
            }
        }
        _ => match app.input.get(x, y) {
            None => {
                app.input.mode = InputFocus::Normal;
            }
            Some(&action) => {
                if let ClickAction::Typing(action) = action {
                    match action {
                        TypingAction::Accept => todo!(),
                        TypingAction::Nothing => todo!(),
                        TypingAction::Exit => todo!(),
                    }
                };
            }
        },
    }
    Ok(())
}

fn draw(x: u16, y: u16, app: &mut App) {
    let x = x - TOOLBOX_WIDTH;

    let size = app.brush.size;
    let tool = app.brush.tool;

    let path = connect_points((x, y), app.canvas.last_pos);
    for (x, y) in path {
        tool.draw(x, y, size, app);
    }
    app.canvas.last_pos = Some((x, y))
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

fn connect_points(start: (u16, u16), end: Option<(u16, u16)>) -> Vec<(u16, u16)> {
    let end = match end {
        None => return vec![start],
        Some(end) if end == start => return vec![start],
        Some(end) => end,
    };

    let start_x = start.0 as i16;
    let start_y = start.1 as i16;
    let end_x = end.0 as i16;
    let end_y = end.1 as i16;

    let x_diff = start_x - end_x;
    let y_diff = start_y - end_y;
    let x_diff_abs = x_diff.abs();
    let y_diff_abs = y_diff.abs();

    let x_is_larger = x_diff_abs > y_diff_abs;

    let x_mod = if x_diff < 0 { 1 } else { -1 };
    let y_mod = if y_diff < 0 { 1 } else { -1 };

    let longer_side = x_diff_abs.max(y_diff_abs);
    let shorter_side = x_diff_abs.min(y_diff_abs);

    let slope = if longer_side == 0 {
        0.0
    } else {
        shorter_side as f64 / longer_side as f64
    };

    let mut out = Vec::with_capacity(longer_side as usize);

    for i in 1..=longer_side {
        let shorter_side_increase = (i as f64 * slope).round() as i16;

        let (x_add, y_add) = if x_is_larger {
            (i, shorter_side_increase)
        } else {
            (shorter_side_increase, i)
        };

        let new_x = start_x + x_add * x_mod;
        let new_y = start_y + y_add * y_mod;

        if let (Ok(x), Ok(y)) = (u16::try_from(new_x), u16::try_from(new_y)) {
            out.push((x, y))
        }
    }

    out
}
