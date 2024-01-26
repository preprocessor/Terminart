use ratatui::layout::Rect;

use super::clicks::ClickAction;

pub type ClickLayer = hashbrown::HashMap<(u16, u16), ClickAction>;

#[repr(u8)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Rename,
    Color,
    Help,
}

#[derive(Default, Debug)]
pub struct Input {
    pub mode: InputMode,
    pub normal: ClickLayer,
    pub typing: ClickLayer,
    pub string_buffer: String,
    pub cursor_pos: usize,
}

impl Input {
    pub fn clear(&mut self) {
        self.normal.clear();
        self.typing.clear();
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&ClickAction> {
        match self.mode {
            InputMode::Normal | InputMode::Help => &self.normal,
            _ => &self.typing,
        }
        .get(&(x, y))
    }

    pub fn register_click(&mut self, area: &Rect, action: ClickAction, mode: InputMode) {
        let (left, top) = (area.x, area.y);
        let right = left + area.width;
        let bottom = top + area.height;

        let page = match mode {
            InputMode::Help | InputMode::Normal => &mut self.normal,
            _ => &mut self.typing,
        };

        for y in top..bottom {
            for x in left..right {
                page.insert((x, y), action);
            }
        }
    }

    pub fn toggle_help(&mut self) {
        if self.mode == InputMode::Help {
            self.mode = InputMode::Normal;
        } else {
            self.mode = InputMode::Help;
        }
    }

    #[rustfmt::skip]    pub fn normal(&mut self) { self.mode = InputMode::Normal; }
    #[rustfmt::skip]    pub fn rename(&mut self) { self.mode = InputMode::Rename; }
    #[rustfmt::skip]    pub fn color(&mut self) { self.mode = InputMode::Color; }

    pub fn add_char(&mut self, ch: char) {
        // if ch.is_ascii() {
        self.string_buffer.insert(self.cursor_pos, ch);
        self.cursor_pos += 1;
        // }
    }

    pub fn backspace(&mut self) {
        self.string_buffer = self
            .string_buffer
            .chars()
            .enumerate()
            .filter(|&(i, _)| i != self.cursor_pos - 1)
            .map(|(_, c)| c)
            .collect();

        self.left();
    }

    pub fn delete(&mut self) {
        self.string_buffer = self
            .string_buffer
            .chars()
            .enumerate()
            .filter(|&(i, _)| i != self.cursor_pos)
            .map(|(_, c)| c)
            .collect();
    }

    pub fn home(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn end(&mut self) {
        self.cursor_pos = self.string_buffer.len();
    }

    pub fn left(&mut self) {
        self.cursor_pos = self.cursor_pos.saturating_sub(1);
    }

    pub fn right(&mut self) {
        self.cursor_pos = (self.cursor_pos + 1).min(self.string_buffer.len());
    }

    pub fn exit(&mut self) {
        self.string_buffer = "".into();
        self.cursor_pos = 0;
        self.mode = InputMode::Normal;
    }

    pub fn accept(&mut self) -> Option<String> {
        if self.string_buffer.is_empty() {
            return None;
        }
        let out: String = self.string_buffer.chars().take(20).collect();
        self.exit();
        Some(out)
    }
}
