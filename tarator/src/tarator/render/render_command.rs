/// # RenderCommand
/// Calls the methods which are defined in render_api from platform/* 

use crate::{
    render::render_api::RenderApi,
    math::F32Vec4
};

pub struct RenderCommand<'a, T: RenderApi> {
    #[allow(unused)]
    render_api: &'a T
}

impl<'a, T: RenderApi> RenderCommand<'a, T> {
    #[allow(unused)]
    fn init(&self) { self.render_api.init(); }
    #[allow(unused)]
    fn set_clear_color(&self, color: &F32Vec4) { self.render_api.set_clear_color(color); }
    #[allow(unused)]
    fn clear(&self) { self.render_api.clear(); }
    #[allow(unused)]
    fn draw_indexed(&self) { self.render_api.draw_indexed(); }
}
