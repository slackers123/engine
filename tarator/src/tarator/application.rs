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
        pub struct $label {
            window: &'static dyn Window,
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
        fn run(&mut self) {
            let deltastart = std::time::Instant::now();
            loop {
                let delta = deltastart.elapsed().as_secs_f64();
                for event in self.window.update() {
                    self.event(event.as_ref(), delta);
                    match event.get_action() {
                        EventAction::WINDOWCLOSE => {                
                            println!("Quitting!");
                            return;
                        },
                        EventAction::APPLICATIONUPDATE => {
                            for layer in self.layer_stack.get_iter_mut() {
                                layer.update_mut(delta);
                            }
                        }
                        _ => {}
                    };
                }
            }
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
