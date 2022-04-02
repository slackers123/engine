/// # WindowProps
/// Provides the Window Trait Implementation with Window Information
pub struct WindowProps {
    #[allow(unused)]
    title: &'static str,
    #[allow(unused)]
    width: u32,
    #[allow(unused)]
    height: u32
}

impl Default for WindowProps {
    fn default() -> WindowProps {
        return WindowProps {
            title: "Tarator Engine",
            width: 1280,
            height: 720
        };
    }
}

/// # Window
/// Implemented in "platform"
pub trait Window {
    fn update();
    fn get_width();
    fn get_height();

    // Window Attributes
    fn set_event_callback();
    fn set_vsync(enabled: bool);
    fn get_vsync_enabled() -> bool;

    fn create(window_props: &WindowProps) -> &Self;
}
