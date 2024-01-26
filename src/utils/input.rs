use ratatui::layout::Rect;

use super::clicks::ClickAction;

pub type ClickLayer = hashbrown::HashMap<(u16, u16), ClickAction>;

#[repr(u8)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum InputFocus {
    #[default]
    Normal,
    Rename,
    Color,
    Help,
}

#[derive(Default, Debug)]
pub struct Input {
    pub mode: InputFocus,
    pub normal: ClickLayer,
    pub typing: ClickLayer,
    pub string_buffer: String,
}

impl Input {
    pub fn clear(&mut self) {
        self.normal.clear();
        self.typing.clear();
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&ClickAction> {
        match self.mode {
            InputFocus::Normal | InputFocus::Help => &self.normal,
            _ => &self.typing,
        }
        .get(&(x, y))
    }

    pub fn register_click(&mut self, area: &Rect, action: ClickAction, mode: InputFocus) {
        let (left, top) = (area.x, area.y);
        let right = left + area.width;
        let bottom = top + area.height;

        let page = match mode {
            InputFocus::Help | InputFocus::Normal => &mut self.normal,
            _ => &mut self.typing,
        };

        for y in top..bottom {
            for x in left..right {
                page.insert((x, y), action);
            }
        }
    }

    pub fn toggle_help(&mut self) {
        if self.mode != InputFocus::Help {
            //
        }
    }
}
