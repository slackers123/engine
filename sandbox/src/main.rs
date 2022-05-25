mod example_layer;
use example_layer::*;

#[macro_use]
extern crate tarator;

use tarator::platform::glfw::glfw_window::GLFWWindow;
use tarator::tarator::window::*;

APPLICATION_DECLARE!(SandboxApplication);
impl Application for SandboxApplication {
    APPLICATION_LAYERIMPL!(SandboxApplication);
    // RUN
    fn new() -> SandboxApplication {
        let mut app = SandboxApplication{
            layer_stack: UPtr::new(LayerStack::new())
        };
        app.push_layer(UPtr::new(ExampleLayer::new()));
        return app;
    }
    fn run(&mut self) {
        let deltastart = std::time::Instant::now();
        let window = &mut GLFWWindow::new(WindowProps::default());
        loop {
            let delta = deltastart.elapsed().as_secs_f64();
            for event in window.update() {
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
}
