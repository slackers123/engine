use crate::renderer::{InitInfo, MainloopInfo};
use crate::renderers::renderer::Renders;

pub struct wow_renderer {
    useless_var: i32,
}

impl Default for wow_renderer {
    fn default() -> Self {
        wow_renderer::new(InitInfo::default())
    }
}

impl wow_renderer {
    pub fn new(info: InitInfo) -> Self {
        wow_renderer {
            useless_var: 123,
        }
    }
}

impl Renders for wow_renderer {
    fn init(&self, info: InitInfo) {
        println!("inited with val: {}", self.useless_var);
    }
    fn main_loop(self, info: MainloopInfo) {
        println!("main loop started with val: {}", self.useless_var);
    }
}