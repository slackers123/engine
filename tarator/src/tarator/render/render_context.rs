use crate::tarator::window::Window;

/// ## RenderContext
/// References Render To Window
/// Implemented in platform/*
pub trait RenderContext<TWindow> where
    TWindow: Window {
    fn new(window: &TWindow) -> Self;
    fn init(&self);
    fn swap_buffers(&self);
}