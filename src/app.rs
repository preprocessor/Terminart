use std::error;

use ahash::AHashMap;
use ratatui::layout::Rect;

use crate::{
    utils::{
        brush::Brush, cell::Cell, charpicker::CharPicker, clicks::ClickAction, layer::Layers,
        palette::Palette, undo::History,
    },
    BRUSH_MAX, BRUSH_MIN,
};

/// Application result type.
pub type Result<T> = color_eyre::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Default, Debug)]
pub struct App {
    pub running: bool,
    pub needs_help: bool,
    pub canvas: Layers,
    pub click_areas: AHashMap<(u16, u16), ClickAction>,
    pub undo_history: History,
    pub palette: Palette,
    pub char_picker: CharPicker,
    pub brush: Brush,
}

impl App {
    /// Constructs a new instance of [`App`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            running: true,
            ..Default::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub const fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_help(&mut self) {
        self.needs_help = !self.needs_help;
    }

    // Area functions {
    pub fn register_click_area(&mut self, area: &Rect, action: ClickAction) {
        let (left, top) = (area.x, area.y);
        let right = left + area.width;
        let bottom = top + area.height;

        for y in top..bottom {
            for x in left..right {
                self.click_areas.insert((x, y), action.clone());
            }
        }
    }
    // }

    // Drawing functions {
    pub fn resize(&mut self, _width: u16, _height: u16) {
        // let new_size = (width * height) as usize;
        // self.canvas.shrink_to(new_size);
        // self.canvas.reserve(new_size);
    }

    // /// This is the alternative to `.draw()` that does not record to history
    // pub fn set_cell(&mut self, x: u16, y: u16, new_cell: Cell) {
    //     let _ = self.drawing.insert((x, y), new_cell);
    // }

    pub fn erase(&mut self, x: i16, y: i16) {
        if x >= 0 && y >= 0 {
            let (x, y) = (x as u16, y as u16);
            let layer = self.canvas.current_layer_mut();
            if let Some(old_cell) = layer.data.remove(&(x, y)) {
                self.undo_history.forget_redo();
                self.undo_history.add_undo(x, y, old_cell, &layer.name);
            }
        }
    }

    pub fn draw(&mut self, x: i16, y: i16) {
        if x >= 0 && y >= 0 {
            let (x, y) = (x as u16, y as u16);
            let layer = self.canvas.current_layer_mut();
            let new_cell = self.brush.as_cell();

            let old_cell = layer.data.insert((x, y), new_cell).unwrap_or(Cell::empty());

            // if let Some(old_cell) = layer.data.insert((x, y), new_cell) {
            //     self.undo_history.add_undo(x, y, old_cell, &layer.name);
            // }

            self.undo_history.add_undo(x, y, old_cell, &layer.name);
            self.undo_history.forget_redo();
        }
    }

    pub fn undo(&mut self) {
        if let Some(undo_page) = self.undo_history.undo() {
            if let Some(layer) = self.canvas.get_layer_mut(undo_page.name.clone()) {
                for ((x, y), cell) in undo_page.data {
                    // if let Some(old_cell) = layer.data.insert((x, y), cell) {
                    //     self.undo_history.add_redo(x, y, old_cell, &undo_page.name);
                    // }
                    let old_cell = layer.data.insert((x, y), cell).unwrap_or(Cell::empty());

                    self.undo_history.add_redo(x, y, old_cell, &undo_page.name);
                }
            }
        }
    }

    pub fn redo(&mut self) {
        if let Some(redo_page) = self.undo_history.redo() {
            if let Some(canvas_layer) = self.canvas.get_layer_mut(redo_page.name.clone()) {
                for ((x, y), cell) in redo_page.data {
                    // if let Some(old_cell) = canvas_layer.data.insert((x, y), cell) {
                    //     self.undo_history.add_redo(x, y, old_cell, &redo_page.name);
                    // }
                    let old_cell = canvas_layer
                        .data
                        .insert((x, y), cell)
                        .unwrap_or(Cell::empty());
                    self.undo_history.add_undo(x, y, old_cell, &redo_page.name);
                }
            }
        }
    }

    pub fn reset(&mut self) {
        // NOTE: Possibly remove this because layers make this a trivial task

        // self.undo_history.past.push(self.canvas.clone());
        // self.undo_history.forget_redo();
        // self.canvas.clear();
    }
    // }

    // Palette functions {
    pub fn palette_next_fg(&mut self) {
        self.brush.fg = self.palette.fg_next();
    }

    pub fn palette_prev_fg(&mut self) {
        self.brush.fg = self.palette.fg_prev();
    }

    pub fn palette_next_bg(&mut self) {
        self.brush.bg = self.palette.bg_next();
    }

    pub fn palette_prev_bg(&mut self) {
        self.brush.bg = self.palette.bg_prev();
    }

    // }

    // Brush functions {
    pub fn brush_down(&mut self, val: u16) {
        self.brush.size = self.brush.size.saturating_sub(val).max(BRUSH_MIN);
    }

    pub fn brush_up(&mut self, val: u16) {
        self.brush.size = (self.brush.size + val).min(BRUSH_MAX);
    }
    // }
}
