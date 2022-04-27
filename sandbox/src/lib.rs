mod example_layer;
mod egui_layer;
use example_layer::*;
use egui_layer::*;

#[macro_use]
extern crate tarator;

use tarator::{
    tarator::{
        application::Application,
        window::{WindowProps, Window},
        core::{UPtr, SPtr, Vector},
        event::*,
        layer::*
    },
    platform::glfw::glfw_window::GLFWWindow
};

APPLICATION_DECLARE!(SandboxApplication);
impl Application for SandboxApplication {
    APPLICATION_LAYERIMPL!(SandboxApplication);
    // RUN
    fn new() -> SandboxApplication {
        let mut app = SandboxApplication{
            window: UPtr::new(GLFWWindow::new(&WindowProps::default())),
            layer_stack: UPtr::new(LayerStack::new())
        };
        app.push_layer(SPtr::new(ExampleLayer::new()));
        app.push_overlay(SPtr::new(EGUILayer::new()));
        return app;
    }
    fn run(&mut self) {
        loop {
            let event: Vector<UPtr<dyn Event>> = self.window.update();
            for event in event {
                self.event(event.as_ref());
                match event.get_action() {
                    EventAction::WINDOWCLOSE => {                
                        println!("Quitting!");
                        return;
                    },
                    EventAction::APPLICATIONUPDATE => {
                        for layer in self.layer_stack.get_iter() {
                            match layer.get_category() {
                                LayerCategory::NONE => layer.update(),
                                _ => ()
                            }
                        }
                    }
                    _ => {}
                };
            }
        }
    }
    fn event(&self, event: &dyn Event) {
        for layer in self.layer_stack.get_iter() {
               layer.event(event);
               if event.get_handled() {
                   break;
               }
        }
    }
}
