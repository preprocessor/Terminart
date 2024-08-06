use super::layers::{Layer, LayerData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryAction {
    LayerAdded(u32),
    LayerRemoved(Layer, usize),
    LayerRenamed(u32, String),
    LayerUp(u32),
    LayerDown(u32),
    Draw(u32, LayerData),
}

#[derive(Debug, Default)]
pub struct History {
    pub past: Vec<HistoryAction>,
    pub future: Vec<HistoryAction>,
    pub partial_draw: Option<LayerData>,
}

impl History {
    pub fn draw(&mut self, id: u32, data: LayerData) {
        self.past.push(HistoryAction::Draw(id, data));
    }

    pub fn add_layer(&mut self, id: u32) {
        self.past.push(HistoryAction::LayerAdded(id));
    }

    pub fn remove_layer(&mut self, layer: Layer, index: usize) {
        self.past.push(HistoryAction::LayerRemoved(layer, index));
    }

    pub fn rename_layer(&mut self, id: u32, old_name: String) {
        self.past.push(HistoryAction::LayerRenamed(id, old_name));
    }

    pub fn layer_up(&mut self, id: u32) {
        self.past.push(HistoryAction::LayerUp(id));
    }

    pub fn layer_down(&mut self, id: u32) {
        self.past.push(HistoryAction::LayerDown(id));
    }

    pub fn forget_redo(&mut self) {
        self.future.clear();
    }

    pub fn add_partial_draw(&mut self, mut old_data: LayerData) {
        if let Some(partial) = self.partial_draw.take() {
            old_data.extend(partial);
        }

        self.partial_draw = Some(old_data);
    }

    pub fn finish_partial_draw(&mut self, layer_id: u32) {
        let Some(partial_draw) = self.partial_draw.take() else {
            return;
        };

        self.draw(layer_id, partial_draw);
    }

    pub fn click_to_partial_draw(&mut self) {
        let last_action = self.past.pop();

        let Some(hist_action) = last_action else {
            return;
        };

        if let HistoryAction::Draw(_, data) = hist_action {
            self.add_partial_draw(data);
        } else {
            self.past.push(hist_action);
        }
    }
}
