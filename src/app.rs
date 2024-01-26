use crate::utils::{brush, cell::Cell, charpicker, input, layer, palette, undo};

/// Application result type.
pub type Result<T> = color_eyre::Result<T, Box<dyn std::error::Error>>;

/// Application.
#[derive(Default, Debug)]
pub struct App {
    pub running: bool,
    pub canvas: layer::Layers,
    pub input: input::Input,
    pub undo_history: undo::History,
    pub palette: palette::Palette,
    pub char_picker: charpicker::CharPicker,
    pub brush: brush::Brush,
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

    // Drawing functions {
    pub fn resize(&mut self, _width: u16, _height: u16) {
        self.input.clear();
        // let new_size = (width * height) as usize;
        // self.canvas.shrink_to(new_size);
        // self.canvas.reserve(new_size);
    }

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

            self.undo_history.add_undo(x, y, old_cell, &layer.name);
            self.undo_history.forget_redo();
        }
    }

    pub fn draw_cell(&mut self, x: i16, y: i16, cell: Cell) {
        if x >= 0 && y >= 0 {
            let (x, y) = (x as u16, y as u16);
            let layer = self.canvas.current_layer_mut();

            let old_cell = layer.data.insert((x, y), cell).unwrap_or(Cell::empty());

            self.undo_history.add_undo(x, y, old_cell, &layer.name);
            self.undo_history.forget_redo();
        }
    }

    pub fn undo(&mut self) {
        if let Some(undo_page) = self.undo_history.undo() {
            let layer = self.canvas.get_layer_mut(&undo_page.name);
            for ((x, y), cell) in undo_page.data {
                let old_cell = layer.data.insert((x, y), cell).unwrap_or(Cell::empty());

                self.undo_history.add_redo(x, y, old_cell, &undo_page.name);
            }
        }
    }

    pub fn redo(&mut self) {
        if let Some(redo_page) = self.undo_history.redo() {
            let canvas_layer = self.canvas.get_layer_mut(&redo_page.name);
            for ((x, y), cell) in redo_page.data {
                let old_cell = canvas_layer
                    .data
                    .insert((x, y), cell)
                    .unwrap_or(Cell::empty());
                self.undo_history.add_undo(x, y, old_cell, &redo_page.name);
            }
        }
    }

    pub fn remove_layer(&mut self) {
        let layer = self.canvas.remove_layer();
        // BUG: This is a very simple solution, that does not care about the order
        //      the redo will always place the layer on top of the stack
        self.undo_history.add_removed_layer(layer);
    }

    pub fn rename_layer(&mut self) {
        if let Some(new_name) = self.input.accept() {
            self.canvas.rename_layer(new_name);
        }
    }

    pub fn reset(&mut self) {
        // NOTE: Possibly remove this because layers make this a trivial task

        while let Some(layer) = self.canvas.layers.pop() {
            self.undo_history.add_removed_layer(layer);
        }
    }

    // Palette functions {
    pub fn brush_next_fg(&mut self) {
        self.brush.fg = self.palette.fg_next();
    }

    pub fn brush_prev_fg(&mut self) {
        self.brush.fg = self.palette.fg_prev();
    }

    pub fn brush_next_bg(&mut self) {
        self.brush.bg = self.palette.bg_next();
    }

    pub fn brush_prev_bg(&mut self) {
        self.brush.bg = self.palette.bg_prev();
    }
}
