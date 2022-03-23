use crate::renderers::gl_renderer;

pub struct InitInfo {

}

impl Default for InitInfo {
    fn default() -> Self {
        InitInfo {}
    }
}

pub struct MainloopInfo {

}

impl Default for MainloopInfo {
    fn default() -> Self {
        MainloopInfo {}
    }
}

pub trait Renders {
    fn init(&self, info: InitInfo);
    fn main_loop(self, info: MainloopInfo);
}

pub mod rendering {
    pub fn init<R: super::Renders>(renderer: &R, info: super::InitInfo){
        renderer.init(info);
    }

    pub fn main_loop<R: super::Renders>(renderer: R, info: super::MainloopInfo) {
        renderer.main_loop(info);
    }
}