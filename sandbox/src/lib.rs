#[macro_use]
extern crate tarator;
use tarator::{tarator::{
    application::Application,
    window::{WindowProps, Window},
    core::{UPtr, SPtr, Vector},
    event::{*, key_event::{KeyEvent, KeyPressedEvent}, application_event::{ApplicationUpdateEvent}},
    layer::{Layer, LayerStack}
}};

struct ExampleLayer {
    name: String
}
impl Layer for ExampleLayer {
    fn new(name: String) -> ExampleLayer {
        return ExampleLayer {
            name: name
        };
    }    
    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn event(&self, event: &dyn Event) {
        if event.is_in_action(EventAction::KEYPRESSED) {
            let key_pressed_event: &KeyPressedEvent = CAST!(event, KeyPressedEvent);
            println!("{}", key_pressed_event.get_key_code());
        }
    }
}

APPLICATION_DECLARE!(SandboxApplication);
impl<TWindow> Application<TWindow> for SandboxApplication<TWindow> where
    TWindow: Window { APPLICATION_LAYERIMPL!(SandboxApplication);
    // RUN
    fn new(window_props: &WindowProps) -> SandboxApplication<TWindow> {
        let mut app = SandboxApplication{
            window: UPtr::new(TWindow::new(window_props)),
            layer_stack: UPtr::new(LayerStack::new())
        };
        app.push_layer(SPtr::new(ExampleLayer::new(String::from("Example Layer"))));
        return app;
    }
    fn run(&mut self) {
        loop {
            let event: Vector<UPtr<dyn Event>> = self.window.update();
            for layer in self.layer_stack.layers.iter() {
                layer.update();
            }
            for event in event {
                match event.get_action() {
                    EventAction::WINDOWCLOSE => return,
                    _ => {}
                }
            }
        }
    }
}
