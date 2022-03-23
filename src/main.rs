use std::io::Cursor;
use crate::renderer::{InitInfo, Renders};

mod renderers;
pub use crate::renderers::{gl_renderer, renderer, wow_renderer};

fn main() {
    let gl = wow_renderer::wow_renderer::default();

    let info = renderer::MainloopInfo {};

    renderer::rendering::init(&gl, InitInfo::default());

    renderer::rendering::main_loop(gl, info);
}