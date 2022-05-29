pub mod buffer;
pub mod vertex_array;

/// ## Render Api
/// Implemented in platform/*
/// methods get called in render_command

use crate::{
    math::F32Vec4,
    window::Window,
    render::vertex_array::*,
    core::*
};

/// # API enum
/// based on chosed api the trait should be executed differently per API
pub enum API {
    NONE = 0,
    WGPU = 1,
    GLIUM = 2
}

/// ## RenderApi
/// Implemented in platform/*
pub trait RenderApi {
    fn init(&self);
    fn set_clear_color(&self, color: &F32Vec4);
    fn clear(&self);
    fn draw_indexed(&self, data: SPtr<dyn VertexArray>);
    fn get_api() -> API;
}

/// ## RenderContext
/// References Render To Window
/// Implemented in platform/*
pub trait RenderContext {
    fn new(window: &dyn Window) -> Self where Self: Sized;
    fn init(&self);
    fn swap_buffers(&self);
}

/// ## RenderCommand
/// Calls the methods which are defined in render_api from platform/* 
pub struct RenderCommand<'a, T: RenderApi> {
    render_api: &'a T
}

impl<'a, T: RenderApi> RenderCommand<'a, T> {
    fn init(&self) { self.render_api.init(); }
    fn set_clear_color(&self, color: &F32Vec4) { self.render_api.set_clear_color(color); }
    fn clear(&self) { self.render_api.clear(); }
    fn draw_indexed(&self, data: SPtr<dyn VertexArray>) { self.render_api.draw_indexed(data); }
}
