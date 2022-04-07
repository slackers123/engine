use crate::tarator::window::{WindowProps, Window};
use winit::event_loop::EventLoop;

/// ## WGPU Implementation of window trait
/// tarator/window.rs
pub struct WinitWindow {
    #[allow(unused)]
    event_loop: EventLoop<()>,
    #[allow(unused)]
    window: winit::window::Window
}

impl Window for WinitWindow {
    fn update(&self) {}
    fn get_width(&self) {}
    fn get_height(&self) {}

    fn set_event_callback(&self) {}
    #[allow(unused)]
    fn set_vsync(&self, enabled: bool) {}
    fn get_vsync_enabled(&self) -> bool { return true; }

    #[allow(unused)]
    fn new(window_props: &WindowProps) -> WinitWindow {
        let event_loop: EventLoop<()> = EventLoop::new();
        let window: winit::window::Window = match winit::window::Window::new(&event_loop) {
            Ok(window) => window,
            Err(os_error) => panic!("Failed To Create Window!")
        };
        return WinitWindow {
            event_loop: event_loop,
            window: window
        };
    }
}
