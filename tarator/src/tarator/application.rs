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
    fn event(&mut self, event: &dyn Event, delta: f64);
    fn push_layer(&mut self, layer: UPtr<dyn Layer>);
    fn push_overlay(&mut self, layer: UPtr<dyn Layer>);
}

#[macro_export]
macro_rules! APPLICATION_DECLARE {
    ($label:tt) => {
        use tarator::tarator::{
            application::Application,
            core::*,
            event::*,
            layer::*
        };
        pub struct $label {
            layer_stack: UPtr<LayerStack>
        }
        fn main() {
            TR_LOG_INIT!();
            TR_INFO!("Initialized Log!\n");
            let mut app = $label::new();
            app.run();
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
        fn event(&mut self, event: &dyn Event, delta: f64) {
            for layer in self.layer_stack.get_iter_mut() {
                layer.event_mut(event, delta);
                if event.get_handled() {
                    break;
                }
            }
        }
    };
}
