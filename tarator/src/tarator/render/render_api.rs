/// # Render Api
/// Implemented in platform/*
/// methods get called in render_command

/// # API enum
/// based on chosed api the trait should be executed differently per API
pub enum API {
    NONE = 0,
    WGPU = 1
}

/// # RenderApi
/// Implemented in platform/*
pub trait RenderApi {
    fn init(&self);
    fn set_clear_color(&self); // missing property "color"
    fn clear(&self);
    fn draw_indexed(&self); // missing property "vertex_array"
    fn get_api() -> API;
}
