#[derive(Default, Debug)]
pub struct TextArea {
    pub buffer: String,
    pub pos: usize,
}

impl TextArea {
    pub fn get(&self) -> Option<String> {
        if self.buffer.is_empty() {
            return None;
        }
        let out: String = self.buffer.chars().take(20).collect();
        Some(out)
    }

    pub fn input(&mut self, ch: char) {
        self.buffer.insert(self.pos, ch);
        self.pos += 1;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
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
}
