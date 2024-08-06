use crate::components::save_load::FileSaveError;

#[derive(Default, Debug)]
pub struct TextArea {
    pub buffer: String,
    pub pos: usize,
    pub error: Option<FileSaveError>,
}

impl TextArea {
    pub fn get(&self) -> Option<String> {
        if self.buffer.is_empty() {
            return None;
        }
        Some(self.buffer.clone())
    }

    pub fn input(&mut self, ch: char, max_len: usize) {
        if self.pos >= max_len {
            return;
        }
        self.buffer.insert(self.pos, ch);
        self.pos += 1;
    }

    pub fn backspace(&mut self) {
        self.buffer = self
            .buffer
            .chars()
            .enumerate()
            .filter(|&(i, _)| i != self.pos - 1)
            .map(|(_, c)| c)
            .collect();

        self.left();
    }

    pub fn delete(&mut self) {
        self.buffer = self
            .buffer
            .chars()
            .enumerate()
            .filter(|&(i, _)| i != self.pos)
            .map(|(_, c)| c)
            .collect();
    }

    pub fn home(&mut self) {
        self.pos = 0;
    }

    pub fn end(&mut self) {
        self.pos = self.buffer.len();
    }

    pub fn left(&mut self) {
        self.pos = self.pos.saturating_sub(1);
    }

    pub fn right(&mut self) {
        self.pos = (self.pos + 1).min(self.buffer.len());
    }

    pub fn clear(&mut self) {
        self.buffer = "".into();
        self.pos = 0;
        self.error = None;
    }
}
