//! Implemented in platform/*

use crate::tarator::{
    event::Event,
    core::{UPtr, Vector}
};
bitflags! {
    pub struct WindowAPI: u8 {
        const NONE = 0;
        const GLFW = BIT!(0);
        const WINIT = BIT!(1);
    }
}
/// ## WindowProps
/// Simple Properties of a window used for creating one
pub struct WindowProps {
    #[allow(unused)]
    pub title: String,
    #[allow(unused)]
    pub width: u32,
    #[allow(unused)]
    pub height: u32,
}
impl Default for WindowProps {
    fn default() -> WindowProps {
        return WindowProps {
            title: String::from("Tarator Engine"),
            width: 1280,
            height: 720
        };
    }
}
/// ## EventCallbackFn
/// Function that gets called on event
pub type EventCallbackFn = fn(event: &dyn Event);
#[allow(unused)] fn empty_fun(event: &dyn Event) {}
pub trait Null {fn null() -> Self;}
impl Null for EventCallbackFn {
    fn null() -> EventCallbackFn {
        let func = empty_fun;
        return func;
    }
}
/// ## Window
/// Implemented in platform/*
/// ```
/// fn update(&mut self) -> dyn Event;
/// fn get_width(&self);
/// fn get_height(&self);
/// fn set_vsync(&mut self, enabled: bool);
/// fn get_vsync_enabled(&self) -> bool;
/// fn new(window_props: &WindowProps) -> Self;
/// ```
pub trait Window {
    /// ### Window::update
    /// makes all updates nececcary for window and returns event
    fn update(&mut self) -> Vector<UPtr<dyn Event>>;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_api(&self) -> WindowAPI;
    fn get_native(&mut self) -> &mut dyn std::any::Any;
    fn is_in_api(&self, api: WindowAPI) -> bool { return self.get_api() == api; }

    // Window Attributes
    // fn set_event_callback(&self, callback: &EventCallbackFn);
    fn set_vsync(&mut self, enabled: bool);
    fn get_vsync_enabled(&self) -> bool;

    fn new(window_props: &WindowProps) -> Self where Self: Sized;

    CASTIMPLTRAIT!();
}
