use super::{cell::Cell, layer::Layer};

#[derive(Debug, Default)]
pub struct History {
    pub past: Vec<Layer>,
    pub future: Vec<Layer>,
}

impl History {
    /// Adds a new page to undo if the current page doesn't match the selected layer
    pub fn try_add_page(&mut self, layer_name: &str) {
        match self.past.last() {
            None => self.past.push(Layer::new(layer_name)),
            Some(page) if !page.data.is_empty() => self.past.push(Layer::new(layer_name)),
            _ => {}
        }
    }

    /// Add a cell to the undo history
    pub fn add_undo(&mut self, x: u16, y: u16, new_cell: Cell, layer_name: &str) {
        if self.past.is_empty() || self.past.last().is_some_and(|l| l.name != layer_name) {
            self.past.push(Layer::new(layer_name));
        }

        #[allow(clippy::unwrap_used)]
        let undo_page = self.past.last_mut().unwrap();

        let value = if undo_page.data.contains_key(&(x, y)) {
            // This is to prevent a line that, in a single drag action,
            // intersects itself; from overwriting the original values
            // in the undo history event
            Cell::empty()
        } else {
            new_cell
        };

        undo_page.data.insert((x, y), value);
    }

    /// Add a cell to the redo history
    pub fn add_redo(&mut self, x: u16, y: u16, new_cell: Cell, layer_name: &str) {
        if self.future.is_empty() || self.future.last().is_some_and(|l| l.name != layer_name) {
            self.future.push(Layer::new(layer_name));
        }

        #[allow(clippy::unwrap_used)]
        self.future
            .last_mut()
            .unwrap()
            .data
            .insert((x, y), new_cell);
    }

    /// Moves a page from undo to redo
    /// Returns that page as an output
    pub fn undo(&mut self) -> Option<Layer> {
        let undo = self.past.pop()?;
        self.future.push(undo.clone());

        Some(undo)
    }

    /// Moves a page from redo to undo
    /// Returns that page as an output
    pub fn redo(&mut self) -> Option<Layer> {
        let redo = self.future.pop()?;
        self.past.push(redo.clone());

        Some(redo)
    }

    pub fn forget_redo(&mut self) {
        self.future.clear();
    }

    pub fn add_removed_layer(&mut self, layer: Layer) {
        self.past.push(layer);
    }
}
