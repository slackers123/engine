#[macro_use]
extern crate tarator;

use tarator::{tarator::{
    application::Application,
    window::{WindowProps, Window},
    core::{UPtr, SPtr, Vector},
    event::{
        *,
        key_event::*,
        mouse_event::*,
        application_event::*
    },
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
        match event.get_action() {
            EventAction::KEYPRESSED => println!("Key Pressed: {}", CAST!(event, KeyPressedEvent).get_key_code()),
            EventAction::KEYRELEASED => println!("Key Released: {}", CAST!(event, KeyReleasedEvent).get_key_code()),
            EventAction::KEYTYPED => println!("Key Typed: {}", CAST!(event, KeyTypedEvent).get_key_code()),
            EventAction::MOUSEKEYPRESSED => println!("Mouse Pressed: {}", CAST!(event, MouseKeyPressedEvent).get_key_code()),
            EventAction::MOUSEKEYRELEASED => println!("Mouse Released: {}", CAST!(event, MouseKeyReleasedEvent).get_key_code()),    
            EventAction::MOUSEMOVED => {
                let temp: &MouseMovedEvent = CAST!(event, MouseMovedEvent);
                println!("Mouse Moved: {}, {}", temp.get_move_x(), temp.get_move_y());
            },  
            EventAction::MOUSESCROLLED => {
                let temp: &MouseScrolledEvent = CAST!(event, MouseScrolledEvent);
                println!("Mouse Scrolled: {}, {}", temp.get_offset_x(), temp.get_offset_y());
            },
            EventAction::WINDOWCLOSE => println!("Closing Window..."),
            EventAction::WINDOWRESIZE => {
                let temp: &WindowResizeEvent = CAST!(event, WindowResizeEvent);
                println!("Window Resize: {}, {}", temp.get_size_x(), temp.get_size_y());
            },
            // EventAction::WINDOWFOCUS => (),
            // EventAction::WINDOWLOSTFOCUS => (),
            // EventAction::WINDOWMOVED => (),
            // EventAction::APPLICATIONTICK => (),
            // EventAction::APPLICATIONUPDATE => (),
            // EventAction::APPLICATIONRENDER => (),
            _ => ()
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
            for event in event {
                self.event(event.as_ref());
                match event.get_action() {
                    EventAction::WINDOWCLOSE => return,
                    EventAction::APPLICATIONUPDATE => {
                        for layer in self.layer_stack.layers.iter() {
                            layer.update();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    fn event(&self, event: &dyn Event) {
        for layer in self.layer_stack.layers.iter() {
               layer.event(event);
               if event.get_handled() {
                   break;
               }
        }

    }
}
