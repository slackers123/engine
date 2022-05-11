use tarator::tarator::{
    layer::*,
    event::{
        *,
        key_event::*,
        mouse_event::*,
        application_event::*
    }
};
pub struct ExampleLayer {
    name: String,
    category: LayerCategory
}
impl Layer for ExampleLayer {
    fn new() -> ExampleLayer {
        return ExampleLayer {
            name: String::from("Example Layer"),
            category: LayerCategory::NONE
        };
    }    
    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn get_category(&self) -> LayerCategory {
        return self.category;
    }
    fn event(&self, event: &dyn Event, delta: f64) {
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
