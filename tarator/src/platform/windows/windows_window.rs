/// # Windows Implementation of the Window trait
/// [TODO] make it do something I guess

use crate::tarator::window;

#[allow(unused)]
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}
};

struct WindowsWindow {
    #[allow(unused)]
    data: WindowData,
    #[allow(unused)]
    event_loop: EventLoop<()>,
    #[allow(unused)]
    window: Window
}

struct WindowData {
    #[allow(unused)]
    title: &'static str,
    #[allow(unused)]
    width: u32,
    #[allow(unused)]
    height: u32,
    #[allow(unused)]
    vsync: bool
}

impl WindowsWindow {
    #[allow(unused)]
    fn init(props: window::WindowProps) {  }
}

impl window::Window for WindowsWindow {
    fn update() {}
    fn get_width() {}
    fn get_height() {}

    // Window Attributes
    fn set_event_callback() {}
    #[allow(unused)]
    fn set_vsync(enabled: bool) {}
    fn get_vsync_enabled() -> bool { return true; }

    fn create(&self, window_props: &window::WindowProps) -> Self {
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