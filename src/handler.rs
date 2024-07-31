use crate::app::{App, Result};
use crate::ui::TOOLBOX_WIDTH;
use crate::utils::cell::Cell;
use crate::utils::clicks::{
    ClickAction, Increment, LayerAction, PickAction, RenameAction, ResetValue, SetValue,
};
use crate::utils::input::{InputMode, MouseMode};
use crate::utils::layer::LayerData;
// use crate::TOOLBOX_WIDTH;

use ansi_style::{BGColor, Color as AColor};
use crossterm::event::MouseEventKind::{Down, Drag, Up};
use crossterm::event::{self, KeyEvent, MouseEvent};
use crossterm::event::{KeyCode, KeyModifiers, MouseButton};
use ratatui::style::Color;

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Terminal events.
#[derive(Clone, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
    /// Paste signal
    Paste(String),
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    #[allow(clippy::expect_used)]
                    if event::poll(timeout).expect("failed to poll new events") {
                        match event::read().expect("unable to read event") {
                            event::Event::Key(e) => sender.send(Event::Key(e)),
                            event::Event::Mouse(e) => sender.send(Event::Mouse(e)),
                            event::Event::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            event::Event::FocusGained => Ok(()),
                            event::Event::FocusLost => Ok(()),
                            event::Event::Paste(s) => sender.send(Event::Paste(s)),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        #[allow(clippy::expect_used)]
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match app.input_capture.mode {
        InputMode::Normal | InputMode::Help => normal_mode_keymaps(key_event, app)?,
        InputMode::Rename => rename_mode_keymaps(key_event, app),
        InputMode::Color => color_mode_keymaps(key_event, app),
    }
    Ok(())
}

fn color_mode_keymaps(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Char('c') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            } else {
                app.input_capture.color_picker.input('c');
            }
        }
        KeyCode::Char(ch) => app.input_capture.color_picker.input(ch),
        KeyCode::Esc => app.input_capture.exit(),
        KeyCode::Tab | KeyCode::Down => app.input_capture.color_picker.tab(),
        KeyCode::BackTab | KeyCode::Up => app.input_capture.color_picker.backtab(),
        KeyCode::Backspace => app.input_capture.color_picker.text.backspace(),
        KeyCode::Delete => app.input_capture.color_picker.text.delete(),
        KeyCode::Left => app.input_capture.color_picker.text.left(),
        KeyCode::Right => app.input_capture.color_picker.text.right(),
        KeyCode::Home => app.input_capture.color_picker.text.home(),
        KeyCode::End => app.input_capture.color_picker.text.end(),
        // KeyCode::Enter => app.apply_rename(),
        _ => {}
    }
}

fn rename_mode_keymaps(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Char('c') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            } else {
                app.input_capture.text_area.input('c');
            }
        }
        KeyCode::Char(ch) => app.input_capture.text_area.input(ch),
        KeyCode::Esc => app.input_capture.exit(),
        KeyCode::Backspace => app.input_capture.text_area.backspace(),
        KeyCode::Delete => app.input_capture.text_area.delete(),
        KeyCode::Left => app.input_capture.text_area.left(),
        KeyCode::Right => app.input_capture.text_area.right(),
        KeyCode::Home => app.input_capture.text_area.home(),
        KeyCode::End => app.input_capture.text_area.end(),
        KeyCode::Enter => app.apply_rename(),
        _ => {}
    }
}

fn normal_mode_keymaps(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match key_event.code {
        // Exit application on `ESC` or `Q`
        KeyCode::Esc | KeyCode::Char('Q') => app.quit(),
        // Exit application on `Ctrl-C`
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('v' | 'V') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                todo!("Add paste keybind");
                // let (old_cells, id) = paste_into_canvas(app, x - TOOLBOX_WIDTH, y)?;
                // app.history.draw(id, old_cells);
                // return Ok(());
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
        // KeyCode::Char('?') => app.input_capture.toggle_help(),
        KeyCode::Char('?') => app.input_capture.toggle_help(),
        // Undo / Redo
        KeyCode::Char('u') => app.undo(),
        KeyCode::Char('U') => app.redo(),
        _ => {}
    }
    Ok(())
}

pub fn handle_mouse_events(event: MouseEvent, app: &mut App) -> Result<()> {
    let x = event.column;
    let y = event.row;

    match app.input_capture.mode {
        InputMode::Color => color_mode_mouse(event, app, x, y),
        InputMode::Rename => rename_mode_mouse(event, app, x, y)?,
        InputMode::Normal | InputMode::Help => normal_mouse_mode(event, app)?,
    }
    Ok(())
}

fn rename_mode_mouse(event: MouseEvent, app: &mut App, x: u16, y: u16) -> Result<()> {
    if event.kind == Down(MouseButton::Left) {
        if let Some(ClickAction::Rename(action)) = app.input_capture.get(x, y) {
            match action {
                RenameAction::Accept => app.apply_rename(),
                RenameAction::Exit => app.input_capture.exit(),
                RenameAction::Nothing => {}
            }
        } else {
            app.input_capture.exit();
            normal_mouse_mode(event, app)?;
        }
    };
    Ok(())
}

fn color_mode_mouse(event: MouseEvent, app: &mut App, x: u16, y: u16) {
    if event.kind == Down(MouseButton::Left) || event.kind == Drag(MouseButton::Left) {
        if let Some(&ClickAction::PickColor(action)) = app.input_capture.get(x, y) {
            match action {
                PickAction::AcceptFG => {
                    app.brush.fg = app.input_capture.color_picker.get_style_color()
                }
                PickAction::AcceptBG => {
                    app.brush.bg = app.input_capture.color_picker.get_style_color()
                }
                PickAction::ReplacePColor(c, i) => app.palette.replace(i, c),
                PickAction::ChangeFocus(new_focus) => {
                    app.input_capture.color_picker.set_attention(new_focus)
                }
                PickAction::Update(color, value) => {
                    app.input_capture.color_picker.set(color, value)
                }
                PickAction::Plus(c) => app.input_capture.color_picker.plus(c),
                PickAction::Minus(c) => app.input_capture.color_picker.minus(c),
                PickAction::New => app.input_capture.color_picker.reset(),
                PickAction::Exit => app.input_capture.exit(),
                PickAction::Nothing => {}
            }
        }
    }
}

fn normal_mouse_mode(event: MouseEvent, app: &mut App) -> Result<()> {
    let x = event.column;
    let y = event.row;

    match event.kind {
        Down(btn) => {
            if let Some(&action) = app.input_capture.get(x, y) {
                let count = match event.modifiers {
                    KeyModifiers::CONTROL => 5,
                    KeyModifiers::ALT => 2,
                    _ => 1,
                };

                match action {
                    ClickAction::Draw => {
                        if btn == MouseButton::Middle {
                            let (old_cells, id) = paste_into_canvas(app, x - TOOLBOX_WIDTH, y)?;
                            app.history.draw(id, old_cells);
                            return Ok(());
                        }

                        app.input_capture.mouse_mode = MouseMode::Click;

                        let drawn_cells = draw_wrapper(x, y, app);
                        let layer_id = app.canvas.current_layer_id();

                        app.history.draw(layer_id, drawn_cells);
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
                        SetValue::Char(c) => app.brush.char = c,
                        SetValue::Reset(rv) => match rv {
                            ResetValue::FG => app.brush.fg = Color::Reset,
                            ResetValue::BG => app.brush.bg = Color::Reset,
                        },
                        SetValue::Color(color) => match btn {
                            MouseButton::Left => app.brush.bg = color,
                            MouseButton::Right => app.brush.fg = color,
                            MouseButton::Middle => match color {
                                c if c == app.brush.fg => app.brush.fg = Color::Reset,
                                c if c == app.brush.bg => app.brush.bg = Color::Reset,
                                _ => {}
                            },
                        },
                    },
                    ClickAction::Layer(action) => match action {
                        LayerAction::Add => {
                            let new_layer_id = app.canvas.add_layer();
                            app.history.add_layer(new_layer_id);
                        }
                        LayerAction::Select(index) => app.canvas.select_layer(index),
                        LayerAction::Remove => app.remove_active_layer(),
                        LayerAction::Rename => app.input_capture.change_mode(InputMode::Rename),
                        LayerAction::MoveUp => {
                            let layer_id = app.canvas.get_active_layer().id;
                            let move_was_sucessful = app.canvas.move_layer_up_by_id(layer_id);
                            if move_was_sucessful {
                                app.history.layer_up(layer_id);
                            }
                        }
                        LayerAction::MoveDown => {
                            let layer_id = app.canvas.get_active_layer().id;
                            let move_was_sucessful = app.canvas.move_layer_down_by_id(layer_id);
                            if move_was_sucessful {
                                app.history.layer_down(layer_id);
                            }
                        }
                        LayerAction::ToggleVis(index) => app.canvas.toggle_visible(index),
                    },
                    ClickAction::PickColor(PickAction::New) => {
                        app.input_capture.change_mode(InputMode::Color)
                    }
                    _ => {}
                }
            }
        }

        Drag(MouseButton::Left | MouseButton::Right) => {
            if let Some(&action) = app.input_capture.get(x, y) {
                if action != ClickAction::Draw {
                    // INFO: If the action isnt a draw action
                    // INFO: return early because we only want draw to respond to drag events
                    return Ok(());
                }

                if app.input_capture.mouse_mode == MouseMode::Click {
                    app.history.click_to_partial_draw();
                }

                app.input_capture.mouse_mode = MouseMode::Drag;

                let old_data = draw_wrapper(x, y, app);

                app.history.add_partial_draw(old_data);
            }
        }
        Up(MouseButton::Left) => {
            if event.modifiers != KeyModifiers::CONTROL {
                app.canvas.last_pos = None;
            }

            if app.input_capture.mouse_mode == MouseMode::Drag {
                let layer_id = app.canvas.current_layer_id();
                app.history.finish_partial_draw(layer_id);
            }

            app.input_capture.mouse_mode = MouseMode::Normal;
        }

        _ => {}
    }
    Ok(())
}

fn draw_wrapper(x: u16, y: u16, app: &mut App) -> LayerData {
    let x = x - TOOLBOX_WIDTH;

    let size = app.brush.size;
    let tool = app.brush.tool;

    let mut old_cells = LayerData::new();

    let path = connect_points((x, y), app.canvas.last_pos);

    for (x, y) in path {
        let mut partial_draw_step = tool.draw(x, y, size, app);

        partial_draw_step.extend(old_cells);

        old_cells = partial_draw_step;
    }

    app.canvas.last_pos = Some((x, y));

    old_cells
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

fn get_drawing_region(app: &mut App) -> Result<(u16, u16, u16, u16, LayerData)> {
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

fn copy_canvas_ansi(app: &mut App) -> Result<()> {
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

fn copy_canvas_text(app: &mut App) -> Result<()> {
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

fn paste_into_canvas(app: &mut App, x: u16, y: u16) -> Result<(LayerData, u32)> {
    let clipboard = cli_clipboard::get_contents()?;
    let mut old_cells = LayerData::new();
    for (dy, row) in clipboard.split('\n').enumerate() {
        for (dx, char) in row.chars().enumerate() {
            let (fx, fy) = (x + dx as u16, y + dy as u16);
            let old_cell = app.insert_at_cell(
                fx,
                fy,
                Cell {
                    char,
                    ..Default::default()
                },
            );
            old_cells.insert((fx, fy), old_cell);
        }
    }

    let active_id = app.canvas.layers[app.canvas.active].id;
    Ok((old_cells, active_id))
}

fn connect_points(start: (u16, u16), end: Option<(u16, u16)>) -> Vec<(u16, u16)> {
    let end = match end {
        Some(end) if end != start => end,
        _ => return vec![start],
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
