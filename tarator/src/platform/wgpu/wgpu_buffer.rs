/// # WGPU Buffer Implementation

use crate::tarator::render::buffer::{BufferLayout, VertexBuffer, IndexBuffer};

pub struct WGPUVertexBuffer {
    render_id: u32,
    layout: BufferLayout
}

impl VertexBuffer for WGPUVertexBuffer {
    fn bind(&self) {}
    fn unbind(&self) {}
    fn get_layout(&self) -> BufferLayout;
    fn set_layout(&self, layout: &BufferLayout);
    fn new(vertices: &f32, size: u32) -> WGPUVertexBuffer {}
}


pub struct WGPUIndexBuffer {
    render_id: u32,
    count: u32
}

impl IndexBuffer for WGPUIndexBuffer {
    fn bind(&self) {}
    fn unbind(&self) {}
    fn get_count(&self) -> u32 {}
    fn new(indices: &u32, size: u32) -> WGPUIndexBuffer {}
}