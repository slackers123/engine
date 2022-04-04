/// # Windows Implementation of the Window trait
/// [TODO] make it do something I guess

use crate::tarator::window::*;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}
};

struct WindowsWindow {
    data: WindowData,
    event_loop: EventLoop<()>,
    window: Window
}

struct WindowData {
    title: &'static mut str,
    width: u32,
    height: u32,
    vsync: bool
}

impl WindowsWindow {
    fn Init(props: WindowProps) {  }
}

impl Window for WindowsWindow {
    fn update() {}
    fn get_width() {}
    fn get_height() {}

    // Window Attributes
    fn set_event_callback() {}
    fn set_vsync(enabled: bool) {}
    fn get_vsync_enabled() -> bool {}

    fn create(window_props: &WindowProps) -> &Self {
        return WindowsWindow {
            data: WindowData {
                title: window_props.title,
                width: window_props.width,
                height: window_props.height,
                /// This below me is currently hard-coded, consider making it alter somehow
                vsync: true
            },
            event_loop: EventLoop::new(),
            window: WindowBuilder::new().build(&self.event_loop).unwrap()
        }
    }
}