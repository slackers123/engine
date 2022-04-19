use crate::tarator::window::Window;

/// ## RenderContext
/// References Render To Window
/// Implemented in platform/*
pub trait RenderContext {
    fn new(window: &dyn Window) -> Self where Self: Sized;
    fn init(&self);
    fn swap_buffers(&self);
}