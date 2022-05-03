/// # Application
/// Here, the application base trait is implemented
/// Use this trait in another crate to create your application

use crate::tarator::{
    layer::*,
    core::*,
    event::Event
};
pub trait Application {
    fn new() -> Self;
    fn run(&mut self);
    fn event(&self, event: &dyn Event);
    fn push_layer(&mut self, layer: UPtr<dyn Layer>);
    fn push_overlay(&mut self, layer: UPtr<dyn Layer>);
}

#[macro_export]
macro_rules! APPLICATION_DECLARE {
    ($label:tt) => {
        pub struct $label {
            window: UPtr<dyn Window>,
            layer_stack: UPtr<LayerStack>
        }
    };
}
#[macro_export]
macro_rules! APPLICATION_LAYERIMPL {
    ($label:tt) => {
        fn push_layer(&mut self, layer: UPtr<dyn Layer>) {
            layer.attach();
            self.layer_stack.push_layer(layer);
        }
        fn push_overlay(&mut self, layer: UPtr<dyn Layer>) {
            layer.attach();
            self.layer_stack.push_overlay(layer);
        }
    };
}
