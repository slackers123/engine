/// # Render
/// calls the render_command methods to do whatever it needs to do

pub mod render_api;
pub mod render_command;
pub mod buffer;


/// # GraphicsContext
/// Swaps Buffers in VertexArray
/// Implemented in platform/*
pub trait GraphicsContext {
    fn init();
    fn swap_buffers();
}