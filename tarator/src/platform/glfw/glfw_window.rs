extern crate glfw;

#[allow(unused)]
use crate::tarator::{
    core::{UPtr, SPtr, Vector},
    render::render_context::RenderContext,
    window::{
        EventCallbackFn, Null,
        Window,
        WindowProps
    },
    event::{
        Event,
        application_event::*
    }
};
mod g {
    pub extern crate glfw;
    pub use glfw::*;
    pub use std::sync::mpsc::Receiver;
}

/// ## GLFWWindowData
struct GLFWWindowData {
    #[allow(unused)]
    title: String,
    width: u32,
    height: u32,
    vsync: bool
}
/// ## GLFWWindow
pub struct GLFWWindow {
    glfw: UPtr<g::Glfw>,
    events: g::Receiver<(f64, g::WindowEvent)>,
    #[allow(unused)]
    window: UPtr<g::Window>,
    // context: &'a dyn RenderContext,
    data: GLFWWindowData
}
impl Window for GLFWWindow {
    fn update(&mut self) -> Vector<UPtr<dyn Event>> {
        let mut return_events: Vector<UPtr<dyn Event>> = Vector::new();
        self.glfw.poll_events();
        #[allow(unused)]
        for (f, event) in g::flush_messages(&self.events) {
            match event {
                g::WindowEvent::Close => {
                    return_events.push(UPtr::new(WindowCloseEvent::default()));
                },
                _ => ()
            }
        }
        return return_events;
    }
    fn get_width(&self) -> u32 { return self.data.width; }
    fn get_height(&self) -> u32 { return self.data.height; }

    // #[allow(unused)]
    // fn set_event_callback(&self, callback: &EventCallbackFn) {}
    fn set_vsync(&mut self, enabled: bool) { self.data.vsync = enabled; }
    fn get_vsync_enabled(&self) -> bool { return self.data.vsync; }

    fn new(window_props: &WindowProps) -> GLFWWindow {
        let mut glfw = g::init(g::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(g::WindowHint::Resizable(true));
        let (mut window, events) = glfw.create_window(
            window_props.width,
            window_props.height,
            window_props.title.as_str(),
            glfw::WindowMode::Windowed,
        ).expect("Failed to create GLFW window.");
        window.set_sticky_keys(true);
        window.set_all_polling(true);
        return GLFWWindow {
            glfw: UPtr::new(glfw),
            events: events,
            window: UPtr::new(window),
            data: GLFWWindowData {
                title: window_props.title.clone(),
                width: window_props.width,
                height: window_props.height,
                vsync: true
            }
        };
    }
}