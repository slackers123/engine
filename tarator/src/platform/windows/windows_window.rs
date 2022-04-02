/// # Windows Implementation of the Window trait
/// [TODO] make it do something I guess

use crate::tarator::window::*;

struct WindowsWindow {
    data: WindowData    
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

    fn create(window_props: &WindowProps) -> &Self {}
}