use crate::tarator::{
    event::Event,
    core::{Vector, SPtr}
};
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
    fn new(name: String) -> Self where Self: Sized;
    // Executes when attaching to LayerStack
    fn attach(&self) {}
    // Executes when detaching from LayerStack
    fn detach(&self) {}
    fn update(&self) {}
    fn gui_update(&self) {}
    #[allow(unused)]
    fn event(&self, event: &dyn Event) {}
    fn get_name(&self) -> String;
}
/// ## LayerStack
/// Manages Layers
pub struct LayerStack {
    pub layers: Vector<SPtr<dyn Layer>>,
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
    pub fn push_layer(&mut self, layer: SPtr<dyn Layer>) {
        self.layers.insert(self.index, layer);
        self.index += 1;
    }
    #[allow(unused)]
    pub fn push_overlay(&mut self, layer: SPtr<dyn Layer>) {
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
}
