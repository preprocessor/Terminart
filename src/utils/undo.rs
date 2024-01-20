use crate::app::Page;

use super::cell::Cell;

#[derive(Debug, Default)]
pub struct History {
    pub past: Vec<Page>,
    pub future: Vec<Page>,
}

impl History {
    pub fn try_add_page(&mut self) {
        match self.past.last() {
            None => self.past.push(Page::default()),
            Some(page) if !page.is_empty() => self.past.push(Page::default()),
            _ => {}
        }
    }

    pub fn add_undo(&mut self, x: u16, y: u16, cell: Cell) {
        if self.past.is_empty() {
            self.past.push(Page::default());
        }

        if let Some(page) = self.past.last_mut() {
            let value = if page.contains_key(&(x, y)) {
                // This is to prevent a line that, in a single drag action,
                // intersects itself; from overwriting the original values
                // in the undo history event
                Cell::empty()
            } else {
                cell
            };

            page.insert((x, y), value);
        }
    }

    pub fn add_redo(&mut self, x: u16, y: u16, value: Cell) {
        if self.future.is_empty() {
            self.future.push(Page::default());
        }

        #[allow(clippy::unwrap_used)]
        self.future.last_mut().unwrap().insert((x, y), value);
    }

    pub fn undo(&mut self) -> Option<Page> {
        let undo = self.past.pop()?;
        self.future.push(undo.clone());

        Some(undo)
    }

    pub fn redo(&mut self) -> Option<Page> {
        let redo = self.future.pop()?;
        self.past.push(redo.clone());

        Some(redo)
    }

    pub fn forget_redo(&mut self) {
        self.future.clear();
    }
}
