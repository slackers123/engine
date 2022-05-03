use crate::{
    tarator::{
        core::*,
        window::*,
        event::{
            Event,
            application_event::*,
            key_event::*,
            mouse_event::*
        }
    },
    platform::glfw::glfw_keycode::*
};
extern crate glfw;
mod g {
    pub extern crate egui_glfw_gl;
    pub use std::sync::mpsc::Receiver;
    pub use egui_glfw_gl::glfw::*;
}
pub use egui_glfw_gl::glfw::Context;
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
        #[allow(unused)]
        for (f, event) in g::flush_messages(&self.events) {
            match event {
                g::WindowEvent::Size(x, y) => {
                    self.data.width = x as u32;
                    self.data.height = y as u32;
                    return_events.push(UPtr::new(WindowResizeEvent::new(x as u32, y as u32)));
                },
                g::WindowEvent::Close => {
                    return_events.push(UPtr::new(WindowCloseEvent::default()));
                },
                g::WindowEvent::Char(keycode) => {
                    TR_INFO!("GLFW keycode as char to u32, NOT to TR_KEYCODE!");
                    return_events.push(UPtr::new(KeyTypedEvent::new(keycode as u32)));
                },
                g::WindowEvent::Key(key, keycode, action, mods) => {
                    match action {
                        g::Action::Press => {
                            return_events.push(UPtr::new(KeyPressedEvent::new(get_tr_keycode(key), 0)));
                        },
                        g::Action::Release => {
                            return_events.push(UPtr::new(KeyReleasedEvent::new(get_tr_keycode(key))));
                        },
                        g::Action::Repeat => {
                            return_events.push(UPtr::new(KeyPressedEvent::new(get_tr_keycode(key), 1)));
                        }
                    };
                },
                g::WindowEvent::MouseButton(mousekey, action, mods) => {
                    match action {
                        g::Action::Press => {
                            return_events.push(UPtr::new(MouseKeyPressedEvent::new(get_tr_mousekeycode(mousekey), 0)));
                        },
                        g::Action::Release => {
                            return_events.push(UPtr::new(MouseKeyReleasedEvent::new(get_tr_mousekeycode(mousekey))));
                        },
                        g::Action::Repeat => {
                            return_events.push(UPtr::new(MouseKeyPressedEvent::new(get_tr_mousekeycode(mousekey), 1)));
                        }
                    };
                },
                g::WindowEvent::Scroll(x, y) => {
                    return_events.push(UPtr::new(MouseScrolledEvent::new(x, y)));
                },
                g::WindowEvent::CursorPos(x, y) => {
                    return_events.push(UPtr::new(MouseMovedEvent::new(x, y)));
                }
                _ => ()
            }
        }
        self.window.swap_buffers();
        self.glfw.poll_events();
        return_events.push(UPtr::new(ApplicationUpdateEvent::default()));
        return return_events;
    }
    fn get_width(&self) -> u32 { return self.data.width; }
    fn get_height(&self) -> u32 { return self.data.height; }
    fn get_api(&self) -> WindowAPI {
        return WindowAPI::GLFW;
    }

    // #[allow(unused)]
    // fn set_event_callback(&self, callback: &EventCallbackFn) {}
    fn set_vsync(&mut self, enabled: bool) { self.data.vsync = enabled; }
    fn get_vsync_enabled(&self) -> bool { return self.data.vsync; }
    CASTIMPL!();

    fn new(window_props: &WindowProps) -> GLFWWindow {
        let mut glfw = g::init(g::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(g::WindowHint::Resizable(true));
        let (mut window, events) = glfw.create_window(
            window_props.width,
            window_props.height,
            window_props.title.as_str(),
            egui_glfw_gl::glfw::WindowMode::Windowed,
        ).expect("Failed to create GLFW window.");
        window.set_sticky_keys(true);
        window.set_all_polling(true);
        window.make_current();
        glfw.set_swap_interval(egui_glfw_gl::glfw::SwapInterval::Sync(1));
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        unsafe{gl::ClearColor(0.455, 0.302, 0.663, 1.0);}
        return GLFWWindow {
            glfw: UPtr::new(glfw),
            events,
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
impl GLFWWindow {
    pub fn get_native(&mut self) -> &mut g::Window {
        return self.window.as_mut();
    }
}
