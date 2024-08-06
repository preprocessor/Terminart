use serde::{Deserialize, Serialize};

use super::cell::Cell;

/// Wrapper type for the HashMap that stores the layer
/// Indexing begins at (1, 1), values below will be ignored
pub type LayerData = hashbrown::HashMap<(u16, u16), Cell>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub id: u32,
    pub data: LayerData,
}

impl Layer {
    pub fn new() -> (Self, u32) {
        let this = Self {
            name: "New Layer".into(),
            visible: true,
            id: alea::u32(),
            data: LayerData::default(),
        };
        let id = this.id;
        (this, id)
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }
}

#[derive(Debug, Clone)]
pub struct Layers {
    pub layers: Vec<Layer>,
    pub last_pos: Option<(u16, u16)>,
    pub active: usize,
    pub id_list: Vec<u32>,
    rendered: Option<LayerData>,
}

impl Default for Layers {
    fn default() -> Self {
        let (layer0, id0) = Layer::new();
        Self {
            layers: vec![layer0],
            active: 0,
            last_pos: None,
            id_list: vec![id0],
            rendered: None,
        }
    }
}

impl Layers {
    /// This checks the [selected] property against the amount of [Layers]
    /// It will add a Layer if necessary
    fn check_self(&mut self) {
        // Before you wreck self
        let len = self.layers.len();

        if self.layers.is_empty() {
            self.add_layer();
        }
        if self.active >= len {
            self.active = len.saturating_sub(1);
        }
    }

    pub fn current_layer_mut(&mut self) -> &mut Layer {
        self.check_self();
        self.queue_render();
        &mut self.layers[self.active]
    }

    pub fn current_layer_name(&mut self) -> &str {
        self.check_self();
        &self.layers[self.active].name
    }

    pub fn current_layer_id(&mut self) -> u32 {
        self.check_self();
        self.layers[self.active].id
    }

    pub fn toggle_visible(&mut self, index: u8) {
        self.queue_render();
        if let Some(layer) = self.layers.get_mut(index as usize) {
            layer.toggle_visible();
        }
    }

    pub fn add_layer(&mut self) -> u32 {
        let (new_layer, new_layer_id) = Layer::new();

        // Id generation is purely random and has a non-zero chance of having duplicate ids, this is a shitty solution to that
        if self.id_list.contains(&new_layer_id) {
            self.add_layer()
        } else {
            self.id_list.push(new_layer_id);
            self.layers.push(new_layer);
            new_layer_id
        }
    }

    pub fn get_mut_layer(&mut self, layer_id: u32) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == layer_id)
    }

    pub fn get_active_layer(&self) -> &Layer {
        &self.layers[self.active]
    }

    pub fn get_layer_name(&self, layer_id: u32) -> Option<String> {
        self.layers
            .iter()
            .find(|l| l.id == layer_id)
            .cloned()
            .map(|l| l.name)
    }

    pub fn get_layer_mut(&mut self, layer_id: u32) -> &mut Layer {
        let index_option = self.layers.iter().position(|l| l.id == layer_id);
        self.queue_render();

        match index_option {
            Some(index) => &mut self.layers[index],
            None => {
                let (new_layer, new_layer_id) = Layer::new();
                self.layers.push(new_layer);
                self.id_list.push(new_layer_id);
                #[allow(clippy::unwrap_used)]
                self.layers.last_mut().unwrap()
            }
        }
    }

    pub fn select_layer(&mut self, index: u8) {
        self.active = index as usize;
    }

    /// Removes the currently selected layer
    pub fn remove_active_layer(&mut self) -> (Layer, usize) {
        let layer = self.layers.remove(self.active);
        let old_index = self.active;
        self.active = self.active.saturating_sub(1);
        self.check_self();
        self.queue_render();
        (layer, old_index)
    }

    /// Returns the lauer identifier and the old name
    pub fn rename_layer(&mut self, new_name: String) -> (u32, String) {
        self.check_self();
        let layer = &mut self.layers[self.active];
        let old_name = layer.name.clone();
        layer.name = new_name;
        (layer.id, old_name)
    }

    pub fn move_layer_up(&mut self) -> u32 {
        let max_index = self.layers.len() - 1;
        let new_index = (self.active + 1).min(max_index);
        self.layers.swap(self.active, new_index);
        self.queue_render();
        self.active = new_index;
        self.layers[new_index].id
    }

    pub fn move_layer_up_by_id(&mut self, layer_id: u32) -> bool {
        let max_index = self.layers.len() - 1;
        let Some(layer_index) = self.layers.iter().position(|l| l.id == layer_id) else {
            return false;
        };
        let new_index = (layer_index + 1).min(max_index);
        if new_index != layer_index {
            self.layers.swap(layer_index, new_index);
            self.active = new_index;
            self.queue_render();
            return true;
        }
        false
    }

    pub fn move_layer_down(&mut self) -> u32 {
        let new_index = self.active.saturating_sub(1);
        self.layers.swap(self.active, new_index);
        self.queue_render();
        self.active = new_index;
        self.layers[new_index].id
    }

    pub fn move_layer_down_by_id(&mut self, layer_id: u32) -> bool {
        let Some(layer_index) = self.layers.iter().position(|l| l.id == layer_id) else {
            return false;
        };
        let new_index = layer_index.saturating_sub(1);
        if new_index != layer_index {
            self.layers.swap(layer_index, new_index);
            self.active = new_index;
            self.queue_render();
            return true;
        }
        false
    }

    pub fn get_display_info(&self) -> Vec<(usize, (String, bool))> {
        self.layers
            .iter()
            .map(|l| (l.name.clone(), l.visible))
            .rev()
            .enumerate()
            .collect()
    }

    pub fn queue_render(&mut self) {
        self.rendered = None;
    }

    /// Combine all of the layers into a final output
    pub fn render(&mut self) -> LayerData {
        self.rendered
            .get_or_insert_with(|| {
                self.layers.iter().filter(|l| l.visible).fold(
                    LayerData::default(),
                    |mut page, layer| {
                        page.extend(layer.data.iter().filter(|&(_, &c)| c != Cell::default()));
                        page
                    },
                )
            })
            .clone()
    }

    pub fn remove_layer_by_id(&mut self, id: u32) {
        self.layers.retain(|l| l.id != id);
        self.id_list.retain(|id0| id0 != &id);
    }

    pub fn insert_layer(&mut self, layer: Layer, pos: usize) {
        let id = layer.id;
        self.id_list.push(id);
        self.layers.insert(pos, layer);
    }

    pub fn add_layer_with_id(&mut self, id: u32) {
        let (mut new_layer, _) = Layer::new();
        new_layer.id = id;

        self.id_list.push(id);
        self.layers.push(new_layer);
    }
}
