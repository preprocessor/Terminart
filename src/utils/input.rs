use ratatui::layout::Rect;

use super::clicks::ClickAction;

pub type ClickLayer = hashbrown::HashMap<(u16, u16), ClickAction>;

#[repr(u8)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Rename,
    Color(ColorField),
    Help,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum ColorField {
    #[default]
    Hex,
    R,
    G,
    B,
}

#[derive(Default, Debug)]
pub struct ColorPicker {
    r: (u8, usize),
    g: (u8, usize),
    b: (u8, usize),
    buffer: (String, usize),
}

#[derive(Default, Debug)]
pub struct TextArea {
    pub buffer: String,
    pub pos: usize,
}

impl TextArea {
    pub fn add_char(&mut self, ch: char) {
        self.buffer.insert(self.pos, ch);
        self.pos += 1;
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.pos = 0;
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

#[derive(Default, Debug)]
pub struct Input {
    pub mode: InputMode,
    pub normal: ClickLayer,
    pub typing: ClickLayer,
    pub text: TextArea,
    pub color: ColorPicker,
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
    #[rustfmt::skip]    pub fn color(&mut self) { self.mode = InputMode::Color(ColorField::Hex); }

    pub fn exit(&mut self) {
        self.text.reset();
        self.typing.clear();
        self.mode = InputMode::Normal;
    }

    pub fn get_text(&self) -> Option<String> {
        if self.text.buffer.is_empty() {
            return None;
        }
        let out: String = self.text.buffer.chars().take(20).collect();
        Some(out)
    }

    pub fn accept(&mut self) -> Option<String> {
        if self.text.buffer.is_empty() {
            return None;
        }
        let out: String = self.text.buffer.chars().take(20).collect();
        self.exit();
        Some(out)
    }
}
