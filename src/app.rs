use crate::components::brush::Brush;
use crate::components::cell::Cell;
use crate::components::charpicker::CharPicker;
use crate::components::history::{History, HistoryAction};
use crate::components::input::InputCapture;
use crate::components::layers::{LayerData, Layers};
use crate::components::palette::Palette;
use crate::ui::TOOLBOX_WIDTH;

/// Application result type.
pub type AppResult<T> = core::result::Result<T, Box<dyn std::error::Error>>;

/// Application.
#[derive(Default, Debug)]
pub struct App {
    pub running: bool,
    pub layers: Layers,
    pub input_capture: InputCapture,
    pub history: History,
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

    pub fn resize(&mut self, width: u16, height: u16) {
        self.input_capture.clear();

        for layer in self.layers.layers.iter_mut() {
            layer
                .data
                .retain(|&(cx, cy), _cell| cx < width && cy < height);
        }
    }

    /// Removes a cell from the current layer and returns the cell value
    pub fn erase(&mut self, x: u16, y: u16) -> Cell {
        let layer = self.layers.current_layer_mut();
        let old_cell = layer.data.remove(&(x, y)).unwrap_or_default();

        self.history.forget_redo();

        old_cell
    }

    pub fn draw(&mut self, x: u16, y: u16) -> LayerData {
        let x = x - TOOLBOX_WIDTH;

        let size = self.brush.size;
        let tool = self.brush.tool;
        let mut old_cells = LayerData::new();

        let path = connect_points((x, y), self.layers.last_pos);

        for (x, y) in path {
            let mut partial_draw_step = tool.draw(x, y, size, self);
            partial_draw_step.extend(old_cells);

            old_cells = partial_draw_step;
        }

        self.layers.last_pos = Some((x, y));

        old_cells
    }

    pub fn put_cell(&mut self, x: u16, y: u16) -> Cell {
        let layer = self.layers.current_layer_mut();
        let new_cell = self.brush.as_cell();

        let old_cell = layer.data.insert((x, y), new_cell).unwrap_or_default();

        self.history.forget_redo();

        old_cell
    }

    pub fn insert_at_cell(&mut self, x: u16, y: u16, cell: Cell) -> Cell {
        let layer = self.layers.current_layer_mut();

        let old_cell = layer.data.insert((x, y), cell).unwrap_or_default();

        self.history.forget_redo();

        old_cell
    }

    pub fn undo(&mut self) {
        let Some(mut action) = self.history.past.pop() else {
            return;
        };

        match action {
            HistoryAction::LayerAdded(id) => {
                self.layers.remove_layer_by_id(id);
            }
            HistoryAction::LayerRemoved(ref layer, index) => {
                self.layers.insert_layer(layer.clone(), index);
            }
            HistoryAction::LayerRenamed(id, old_name) => {
                let layer = self.layers.get_layer_mut(id);
                let current_name = layer.name.clone();

                layer.name = old_name;

                action = HistoryAction::LayerRenamed(id, current_name);
            }
            HistoryAction::Draw(layer_id, ref draw_data) => {
                let mut old_data = LayerData::new();
                for (&pos, &cell) in draw_data {
                    let cell_op = self.layers.get_layer_mut(layer_id).data.insert(pos, cell);

                    if let Some(cell) = cell_op {
                        old_data.insert(pos, cell);
                    }
                }
                action = HistoryAction::Draw(layer_id, old_data);
            }
            HistoryAction::LayerUp(layer_id) => {
                let _ = self.layers.move_layer_down_by_id(layer_id);
            }
            HistoryAction::LayerDown(layer_id) => {
                let _ = self.layers.move_layer_up_by_id(layer_id);
            }
        }

        self.history.future.push(action);
        self.layers.queue_render();
    }

    pub fn redo(&mut self) {
        let Some(mut action) = self.history.future.pop() else {
            return;
        };

        match action {
            HistoryAction::LayerAdded(id) => self.layers.add_layer_with_id(id),
            HistoryAction::LayerRemoved(ref layer, _index) => {
                self.layers.remove_layer_by_id(layer.id);
            }
            HistoryAction::LayerRenamed(id, name) => {
                let layer = self.layers.get_layer_mut(id);
                let current_name = layer.name.clone();
                layer.name = name;

                action = HistoryAction::LayerRenamed(id, current_name);
            }
            HistoryAction::Draw(layer_id, ref draw_data) => {
                let mut old_data = LayerData::new();
                for (&pos, &cell) in draw_data {
                    let cell_op = self.layers.get_layer_mut(layer_id).data.insert(pos, cell);

                    if let Some(cell) = cell_op {
                        old_data.insert(pos, cell);
                    }
                }
                action = HistoryAction::Draw(layer_id, old_data);
            }
            HistoryAction::LayerUp(layer_id) => {
                self.layers.move_layer_up_by_id(layer_id);
            }
            HistoryAction::LayerDown(layer_id) => {
                self.layers.move_layer_down_by_id(layer_id);
            }
        }

        self.history.past.push(action);
        self.layers.queue_render();
    }

    pub fn remove_active_layer(&mut self) {
        let (layer, index) = self.layers.remove_active_layer();
        self.history.remove_layer(layer, index);
    }

    pub fn apply_rename(&mut self) -> Option<()> {
        let new_name = self.input_capture.text_area.get()?;
        let (id, old_name) = self.layers.rename_active_layer(new_name);
        self.history.rename_layer(id, old_name);
        Some(())
    }

    pub fn reset(&mut self) {
        self.layers = Layers::default();
        self.history = History::default();
        self.palette = Palette::default();
        self.brush = Brush::default();
        self.layers.queue_render();
    }
}

fn connect_points(start: (u16, u16), end: Option<(u16, u16)>) -> Vec<(u16, u16)> {
    let Some(end) = end else {
        return vec![start];
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
