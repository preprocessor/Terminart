use ratatui::layout::Rect;

pub mod color;
pub mod text;

use super::clicks::{ClickAction, PickAction, RenameAction};

pub type ClickLayer = hashbrown::HashMap<(u16, u16), ClickAction>;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Rename,
    Color,
    Help,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum MouseMode {
    #[default]
    Normal,
    Click,
    Drag,
}

#[derive(Default, Debug)]
pub struct InputCapture {
    pub mode: InputMode,
    pub normal_input: ClickLayer,
    pub text_input: ClickLayer,
    pub text_area: text::TextArea,
    pub color_picker: color::ColorPicker,
    pub mouse_mode: MouseMode,
}

impl InputCapture {
    pub fn clear(&mut self) {
        self.normal_input.clear();
        self.text_input.clear();
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&ClickAction> {
        match self.mode {
            InputMode::Normal | InputMode::Help => &self.normal_input,
            _ => &self.text_input,
        }
        .get(&(x, y))
    }

    pub fn register_click(&mut self, area: &Rect, action: ClickAction, mode: InputMode) {
        let (left, top) = (area.x, area.y);
        let right = left + area.width;
        let bottom = top + area.height;

        let page = match mode {
            InputMode::Help | InputMode::Normal => &mut self.normal_input,
            _ => &mut self.text_input,
        };

        for y in top..bottom {
            for x in left..right {
                page.insert((x, y), action);
            }
        }
    }

    pub fn click_mode_normal(&mut self, area: &Rect, action: ClickAction) {
        let (left, top) = (area.x, area.y);
        let right = left + area.width;
        let bottom = top + area.height;

        for y in top..bottom {
            for x in left..right {
                self.normal_input.insert((x, y), action);
            }
        }
    }

    pub fn click_mode_colorpicker(&mut self, area: &Rect, action: PickAction) {
        let (left, top) = (area.x, area.y);
        let right = left + area.width;
        let bottom = top + area.height;

        for y in top..bottom {
            for x in left..right {
                self.text_input
                    .insert((x, y), ClickAction::PickColor(action));
            }
        }
    }

    pub fn click_mode_rename(&mut self, area: &Rect, action: RenameAction) {
        let (left, top) = (area.x, area.y);
        let right = left + area.width;
        let bottom = top + area.height;

        for y in top..bottom {
            for x in left..right {
                self.text_input.insert((x, y), ClickAction::Rename(action));
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

    pub fn change_mode(&mut self, new_mode: InputMode) {
        if self.mode == new_mode {
            return;
        }

        if matches!(new_mode, InputMode::Rename | InputMode::Color) {
            self.text_input.clear();
        }

        self.mode = new_mode;
    }

    pub fn exit(&mut self) {
        self.text_area.reset();
        self.color_picker.reset();
        self.change_mode(InputMode::Normal)
    }

    pub fn accept(&mut self) -> Option<String> {
        if self.text_area.buffer.is_empty() {
            return None;
        }
        let out: String = self.text_area.buffer.chars().take(20).collect();
        self.exit();
        Some(out)
    }
}
