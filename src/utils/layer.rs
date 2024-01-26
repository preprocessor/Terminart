use super::cell::Cell;

pub type Page = hashbrown::HashMap<(u16, u16), Cell>;

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
    pub last_pos: Option<(u16, u16)>,
    pub selected: usize,
}

impl Default for Layers {
    fn default() -> Self {
        Self {
            layers: vec![Layer::new("Layer 1")],
            selected: 0,
            last_pos: None,
        }
    }
}

impl Layers {
    /// Before you wreck self
    /// This checks the [current] property against the amount of [Layers]
    /// It will add a Layer if necessary
    fn check_self(&mut self) {
        let len = self.layers.len();

        if self.layers.is_empty() {
            self.add_layer(None);
        }
        if self.selected >= len {
            self.selected = len.saturating_sub(1);
        }
    }

    pub fn current_layer_visibile(&mut self) -> bool {
        self.check_self();
        self.layers[self.selected].show
    }

    pub fn current_layer_mut(&mut self) -> &mut Layer {
        self.check_self();
        &mut self.layers[self.selected]
    }

    pub fn current_layer_name(&mut self) -> &str {
        self.check_self();
        &self.layers[self.selected].name
    }

    pub fn toggle_show(&mut self, index: u8) {
        if let Some(layer) = self.layers.get_mut(index as usize) {
            layer.toggle_show();
        }
    }

    pub fn add_layer(&mut self, name_mod: Option<usize>) {
        let count_mod = name_mod.unwrap_or(0);
        let new_count = self.layers.len() + 1 + count_mod;

        let new_layer_name = format!("Layer {}", new_count);

        if self.layers.iter().any(|l| l.name == new_layer_name) {
            self.add_layer(Some(count_mod + 1))
        } else {
            self.layers.push(Layer::new(new_layer_name));
        }
    }

    pub fn get_layer_mut(&mut self, target_name: &str) -> &mut Layer {
        let layer_index = self.layers.iter().position(|l| l.name == target_name);

        match layer_index {
            Some(index) => &mut self.layers[index],
            None => {
                self.layers.push(Layer::new(target_name));
                #[allow(clippy::unwrap_used)]
                self.layers.last_mut().unwrap()
            }
        }
    }

    pub fn select_layer(&mut self, index: u8) {
        self.selected = index as usize;
    }

    /// Removes the currently selected layer
    pub fn remove_layer(&mut self) -> Layer {
        let layer = self.layers.remove(self.selected);
        self.selected = self.selected.saturating_sub(1);
        self.check_self();
        layer
    }

    pub fn rename_layer(&mut self, new_name: String) {
        self.check_self();
        self.layers[self.selected].name = new_name;
    }

    pub fn move_layer_up(&mut self) {
        let max_index = self.layers.len() - 1;
        let new_index = (self.selected + 1).min(max_index);
        self.layers.swap(self.selected, new_index);
        self.selected = new_index;
    }

    pub fn move_layer_down(&mut self) {
        let new_index = self.selected.saturating_sub(1);
        self.layers.swap(self.selected, new_index);
        self.selected = new_index;
    }

    pub fn get_info(&self) -> Vec<(String, bool)> {
        self.layers
            .iter()
            .map(|l| (l.name.clone(), l.show))
            .collect()
    }

    /// Combine all of the layers to a final output
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
