use ratatui::layout::Rect;

pub mod color;
pub mod text;

use super::clicks::ClickAction;

pub type ClickLayer = hashbrown::HashMap<(u16, u16), ClickAction>;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Rename,
    Color,
    Help,
    Export,
    Save,
    Exit,
    TooSmall,
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
    pub popup_layer: ClickLayer,
    pub text_area: text::TextArea,
    pub color_picker: color::ColorPicker,
    pub last_file_name: Option<String>,
    pub mouse_mode: MouseMode,
}

impl InputCapture {
    pub fn clear(&mut self) {
        self.normal_input.clear();
        self.popup_layer.clear();
        self.text_area.clear();
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&ClickAction> {
        match self.mode {
            InputMode::Normal | InputMode::Help => &self.normal_input,
            _ => &self.popup_layer,
        }
        .get(&(x, y))
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

    pub fn click_mode_popup(&mut self, area: &Rect, action: ClickAction) {
        let (left, top) = (area.x, area.y);
        let right = left + area.width;
        let bottom = top + area.height;

        for y in top..bottom {
            for x in left..right {
                self.popup_layer.insert((x, y), action);
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

        if !matches!(new_mode, InputMode::Normal | InputMode::Help) {
            self.text_area.clear();
            self.color_picker.reset();
            self.popup_layer.clear();
        }

        self.mode = new_mode;
    }

    pub fn exit(&mut self) {
        self.text_area.clear();
        self.color_picker.reset();
        self.popup_layer.clear();
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
