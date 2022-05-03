mod example_layer;
use example_layer::*;

#[macro_use]
extern crate tarator;

use tarator::{
    tarator::{
        application::Application,
        window::{WindowProps, Window},
        core::*,
        event::*,
        layer::*
    },
    platform::glfw::{
        glfw_window::GLFWWindow,
        glfw_egui::GLFWEGUILayer
    }
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
        app.push_layer(UPtr::new(ExampleLayer::new()));
        let gui = UPtr::new(GLFWEGUILayer::new_glfw_egui(CASTMUT!(app.window.as_mut(), GLFWWindow), |ui| {
            ui.separator();
            ui.label("Tarator EGUI RUNNING!");
            if ui.button("Click Me").clicked() {
                println!("Clicked");
            }
        }));
        app.push_layer(gui);
        return app;
    }
    fn run(&mut self) {
        let deltastart = std::time::Instant::now();
        loop {
            let delta = deltastart.elapsed().as_secs_f64();
            let event: Vector<UPtr<dyn Event>> = self.window.update();
            for event in event {
                self.event(event.as_ref());
                match event.get_action() {
                    EventAction::WINDOWCLOSE => {                
                        println!("Quitting!");
                        return;
                    },
                    EventAction::APPLICATIONUPDATE => {
                        for layer in self.layer_stack.get_iter_mut() {
                            layer.update(delta);
                            layer.update_mut(delta);
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
