//! # Window
//! Provides the Window Trait Implementation with Window Information
//! Implemented in platform/*

use crate::tarator::{
    event::Event,
    core::{SPtr}    
};

/// ## WindowProps
/// Simple Properties of a window used for creating one
pub struct WindowProps {
    #[allow(unused)]
    pub title: String,
    #[allow(unused)]
    pub width: u32,
    #[allow(unused)]
    pub height: u32
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
pub type EventCallbackFn<'a, T> = &'a fn(event: &T);
/// ## Window
/// Implemented in platform/*
pub trait Window {
    fn update(&mut self) -> SPtr<dyn Event>;
    fn get_width(&self);
    fn get_height(&self);

    // Window Attributes
    // fn set_event_callback<'a, TEvent: Event>(&self, callback: EventCallbackFn<'a, TEvent>);
    fn set_vsync(&mut self, enabled: bool);
    fn get_vsync_enabled(&self) -> bool;

    fn new(window_props: &WindowProps) -> Self;
}
