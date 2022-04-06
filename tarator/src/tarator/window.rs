/// # WindowProps
/// Provides the Window Trait Implementation with Window Information
/// Implemented in platform/*

/// ## WindowProps
pub struct WindowProps {
    #[allow(unused)]
    pub title: &'static str,
    #[allow(unused)]
    pub width: u32,
    #[allow(unused)]
    pub height: u32
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

/// ## Window
/// Implemented in platform/*
pub trait Window {
    fn update(&self);
    fn get_width(&self);
    fn get_height(&self);

    // Window Attributes
    fn set_event_callback(&self);
    fn set_vsync(&self, enabled: bool);
    fn get_vsync_enabled(&self) -> bool;

    fn new(window_props: &WindowProps) -> Self;
}
