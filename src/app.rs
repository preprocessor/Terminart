use crate::utils::{
    brush,
    cell::Cell,
    charpicker, input,
    layer::{self, LayerData},
    palette,
    undo::{self, HistoryAction},
};

/// Application result type.
pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

/// Application.
#[derive(Default, Debug)]
pub struct App {
    pub running: bool,
    pub canvas: layer::Layers,
    pub input_capture: input::InputCapture,
    pub history: undo::History,
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
    pub fn resize(&mut self, width: u16, height: u16) {
        self.input_capture.clear();

        for layer in self.canvas.layers.iter_mut() {
            layer
                .data
                .retain(|&(cx, cy), _cell| cx < width && cy < height);
        }
    }

    /// Removes a cell from the current layer and returns the cell value
    pub fn erase(&mut self, x: u16, y: u16) -> Cell {
        let layer = self.canvas.current_layer_mut();
        let old_cell = layer.data.remove(&(x, y)).unwrap_or_default();

        self.history.forget_redo();

        old_cell
    }

    pub fn draw(&mut self, x: u16, y: u16) -> Cell {
        let layer = self.canvas.current_layer_mut();
        let new_cell = self.brush.as_cell();

        let old_cell = layer.data.insert((x, y), new_cell).unwrap_or_default();

        self.history.forget_redo();

        old_cell
    }

    pub fn insert_at_cell(&mut self, x: u16, y: u16, cell: Cell) -> Cell {
        let layer = self.canvas.current_layer_mut();

        let old_cell = layer.data.insert((x, y), cell).unwrap_or_default();

        // self.undo_history.add_undo(x, y, old_cell, &layer.name);
        // TODO: add undo functionality
        self.history.forget_redo();

        old_cell
    }

    pub fn undo(&mut self) {
        let Some(mut action) = self.history.past.pop() else {
            return;
        };

        match action {
            HistoryAction::LayerAdded(id) => {
                self.canvas.remove_layer_by_id(id);
                self.canvas.queue_render();
            }
            HistoryAction::LayerRemoved(ref layer, index) => {
                self.canvas.insert_layer(layer.clone(), index);
                self.canvas.queue_render();
            }
            HistoryAction::LayerRenamed(id, old_name) => {
                let layer = self.canvas.get_layer_mut(id);
                let current_name = layer.name.clone();

                layer.name = old_name;

                action = HistoryAction::LayerRenamed(id, current_name);
            }
            HistoryAction::Draw(layer_id, ref draw_data) => {
                let mut old_data = LayerData::new();
                for (&pos, &cell) in draw_data {
                    let cell_op = self.canvas.get_layer_mut(layer_id).data.insert(pos, cell);

                    if let Some(cell) = cell_op {
                        old_data.insert(pos, cell);
                    }
                }
                action = HistoryAction::Draw(layer_id, old_data);
            }
            HistoryAction::LayerUp(layer_id) => {
                let _ = self.canvas.move_layer_down_by_id(layer_id);
            }
            HistoryAction::LayerDown(layer_id) => {
                let _ = self.canvas.move_layer_up_by_id(layer_id);
            }
        }

        self.history.future.push(action);
        self.canvas.queue_render();
    }

    pub fn redo(&mut self) {
        let Some(mut action) = self.history.future.pop() else {
            return;
        };

        match action {
            HistoryAction::LayerAdded(id) => {
                self.canvas.add_layer_with_id(id);
            }
            HistoryAction::LayerRemoved(ref layer, _index) => {
                self.canvas.remove_layer_by_id(layer.id);
            }
            HistoryAction::LayerRenamed(id, name) => {
                let layer = self.canvas.get_layer_mut(id);
                let current_name = layer.name.clone();
                layer.name = name;

                action = HistoryAction::LayerRenamed(id, current_name);
            }
            HistoryAction::Draw(layer_id, ref draw_data) => {
                let mut old_data = LayerData::new();
                for (&pos, &cell) in draw_data {
                    let cell_op = self.canvas.get_layer_mut(layer_id).data.insert(pos, cell);

                    if let Some(cell) = cell_op {
                        old_data.insert(pos, cell);
                    }
                }
                action = HistoryAction::Draw(layer_id, old_data);
            }
            HistoryAction::LayerUp(layer_id) => {
                let _ = self.canvas.move_layer_up_by_id(layer_id);
            }
            HistoryAction::LayerDown(layer_id) => {
                let _ = self.canvas.move_layer_down_by_id(layer_id);
            }
        }

        self.history.past.push(action);
        self.canvas.queue_render();
    }

    pub fn remove_active_layer(&mut self) {
        let (layer, index) = self.canvas.remove_active_layer();
        self.history.remove_layer(layer, index);
    }

    pub fn apply_rename(&mut self) {
        if let Some(new_name) = self.input_capture.text_area.get() {
            let (id, old_name) = self.canvas.rename_layer(new_name);
            self.history.rename_layer(id, old_name);
            self.input_capture.exit();
        }
    }

    pub fn reset(&mut self) {
        while let Some(layer) = self.canvas.layers.pop() {
            self.history.remove_layer(layer, self.canvas.layers.len());
        }

        self.canvas.queue_render();
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
