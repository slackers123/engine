use crate::tarator::{
    event::Event,
    core::*
};
bitflags! {
    pub struct LayerCategory: u8 {
        const NONE = 0;
        const GUI = BIT!(0);
    }
}
/// ## Layer
/// ```
/// // Necessary Implementations:
/// fn new(name: String) -> Self;
/// fn get_name(&self) -> String;
/// // Optional Implementations:
/// fn attach(&self);
/// fn detach(&self);
/// fn update(&self);
/// fn gui_update(&self);
/// fn event(&self, event: &dyn Event);
/// ```
pub trait Layer {
    fn new() -> Self where Self: Sized;
    // Executes when attaching to LayerStack
    fn attach(&self) {}
    // Executes when detaching from LayerStack
    fn detach(&self) {}
    #[allow(unused)]
    fn update(&self, delta: f64) {}
    #[allow(unused)]
    fn update_mut(&mut self, delta: f64) {
        self.update(delta);
    }
    #[allow(unused)]
    fn event(&self, event: &dyn Event, delta: f64) {}
    #[allow(unused)]
    fn event_mut(&mut self, event: &dyn Event,  delta: f64) {
        self.event(event, delta)
    }
    fn get_name(&self) -> String;
    fn get_category(&self) -> LayerCategory;
    fn is_in_category(&self, category: LayerCategory) -> bool { return self.get_category() == category; }
}
/// ## LayerStack
/// Manages Layers
pub struct LayerStack {
    layers: Vector<UPtr<dyn Layer>>,
    index: usize
}
impl LayerStack {
    #[allow(unused)]
    pub fn new() -> LayerStack {
        return LayerStack {
            layers: Vector::new(),
            index: 0
        }
    }
    #[allow(unused)]
    pub fn push_layer(&mut self, layer: UPtr<dyn Layer>) {
        self.layers.insert(self.index, layer);
        self.index += 1;
    }
    #[allow(unused)]
    pub fn push_overlay(&mut self, layer: UPtr<dyn Layer>) {
        self.layers.insert(0, layer);
    }
    #[allow(unused)]
    pub fn pop_layer(&mut self, name: String) {
        if let Some(index) = self.layers.iter().position(|element| element.get_name() == name) {
            self.layers.remove(index);
            self.index -= 1;
        }
    }
    #[allow(unused)]
    pub fn pop_overlay(&mut self, name: String) {
        if let Some(index) = self.layers.iter().position(|element| element.get_name() == name) {
            self.layers.remove(index);
        }
    }
    #[allow(unused)]
    pub fn get_iter_mut(&mut self) -> std::slice::IterMut<UPtr<dyn Layer>> {
        return self.layers.iter_mut();
    }
    #[allow(unused)]
    pub fn get_iter(&self) -> std::slice::Iter<UPtr<dyn Layer>> {
        return self.layers.iter();
    }
}

