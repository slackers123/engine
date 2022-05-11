mod example_layer;
use example_layer::*;

#[macro_use]
extern crate tarator;

use tarator::{
    tarator::{
        application::Application,
        window::*,
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
            window: &GLFWWindow::new(&WindowProps::default()),
            layer_stack: UPtr::new(LayerStack::new())
        };
        let gui = UPtr::new(GLFWEGUILayer::new_glfw_egui(CASTMUT!(app.window, GLFWWindow), |gui| {
            gui.new_window("Tarator", |ui| {
                ui.label("Test");
                ui.button("Press Me").clicked();
            }); 
            gui.new_window("Window 2", |ui| {
                ui.code("Test");
            });
            println!("{}", gui.ctx.wants_pointer_input());
        }));
        app.push_layer(gui);
        return app;
    }
}
