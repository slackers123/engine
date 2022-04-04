/// # RenderCommand
/// Calls the methods defined in render_api from platform/* 

use crate::tarator::render::render_api::RenderApi;

pub struct RenderCommand<T> where T: 'static + RenderApi {
    #[allow(unused)]
    render_api: &'static T
}

impl<T> RenderCommand<T> where T: 'static + RenderApi {
    #[allow(unused)]
    fn init(&self) { self.render_api.init(); }
    #[allow(unused)]
    fn set_clear_color(&self) { self.render_api.set_clear_color(); }
    #[allow(unused)]
    fn clear(&self) { self.render_api.clear(); }
    #[allow(unused)]
    fn draw_indexed(&self) { self.render_api.draw_indexed(); }
}
