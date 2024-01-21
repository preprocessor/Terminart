use ahash::AHashMap;

use super::cell::Cell;

pub type Page = AHashMap<(u16, u16), Cell>;

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub show: bool,
    pub data: Page,
}

impl Layer {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        let name = String::from(name.as_ref());
        Self {
            name,
            show: true,
            data: Page::default(),
        }
    }

    pub fn toggle_show(&mut self) {
        self.show = !self.show;
    }
}

#[derive(Debug, Clone)]
pub struct Layers {
    pub layers: Vec<Layer>,
    pub current: usize,
}

impl Default for Layers {
    fn default() -> Self {
        Self {
            layers: vec![Layer::new("Layer 1")],
            current: 0,
        }
    }
}

impl Layers {
    fn check_current_pos(&mut self) {
        let len = self.layers.len();
        if self.current >= len {
            self.current = len - 1;
        }
    }

    pub fn current_layer_mut(&mut self) -> &mut Layer {
        self.check_current_pos();
        &mut self.layers[self.current]
    }

    pub fn current_layer_name(&self) -> &str {
        &self.layers[self.current].name
    }

    // pub fn toggle_show(&mut self, layer_name: String) {
    //     if let Some(layer) = self.layers.iter_mut().find(|l| l.name == layer_name) {
    //         layer.toggle_show();
    //     }
    // }

    pub fn toggle_show(&mut self, index: usize) {
        if let Some(layer) = self.layers.get_mut(index) {
            layer.toggle_show();
        }
    }

    // pub fn remove_layer(&mut self, target_name: String) {
    //     self.layers.retain(|l| l.name != target_name);
    //
    //     self.check_current_pos();
    // }

    pub fn remove_layer(&mut self, index: usize) {
        self.layers.remove(index);
        self.check_current_pos();
    }

    pub fn add_layer(&mut self, name_mod: Option<usize>) {
        let new_count = self.layers.len() + 1 + name_mod.unwrap_or(0);

        let new_layer_name = format!("Layer {}", new_count);

        if self.layers.iter().any(|l| l.name == new_layer_name) {
            self.add_layer(Some(new_count + 1))
        } else {
            self.layers.push(Layer::new(new_layer_name));
        }
    }

    pub fn get_layer_mut(&mut self, target_name: String) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.name == target_name)
    }

    // pub fn select_layer(&mut self, target_name: String) {
    //     if let Some(index) = self
    //         .layers
    //         .iter()
    //         .enumerate()
    //         .filter(|(_, l)| l.name == target_name)
    //         .map(|(i, _)| i)
    //         .next()
    //     {
    //         self.current = index;
    //     }
    // }
    pub fn select_layer(&mut self, index: usize) {
        self.current = index;
    }

    pub fn render(&self) -> Page {
        self.layers
            .iter()
            .filter(|l| l.show)
            .fold(Page::default(), |mut page, layer| {
                page.extend(layer.data.iter().filter(|&(_, &c)| c != Cell::empty()));
                page
            })
    }
}
