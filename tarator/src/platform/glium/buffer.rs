use crate::{
    render::{
        buffer::*,
        *
    },
    core::*
};

#[derive(Clone, Copy)]
pub struct GliumVertex {
    pos: [f32; 2]
}
implement_vertex!(GliumVertex, pos);

pub struct GliumVertexBuffer {
    layout: Option<BufferLayout>,
    pub buffer: glium::VertexBuffer<GliumVertex>
}

impl VertexBuffer for GliumVertexBuffer {
    fn bind(&self) {}
    fn unbind(&self) {}
    fn get_layout(&self) -> &Option<BufferLayout> {
        return &self.layout; 
    }
    fn set_layout(&mut self, layout: &BufferLayout) {
        self.layout = Some(layout.clone());
    }

    fn get_api(&self) -> API {
        return API::GLIUM; 
    }
    
    CASTIMPL!();
}

impl GliumVertexBuffer {
    pub fn new(ctx: SPtr<glium::backend::Context>, data: &[GliumVertex], size: u32) -> GliumVertexBuffer {
        return GliumVertexBuffer {
            layout: None,
            buffer: glium::VertexBuffer::new(&ctx, data).expect("Failed To Init Glium Vertex Buffer")
        };
    }   
}


pub struct GliumIndexBuffer {
    count: u32,
    pub buffer: glium::IndexBuffer<u32>
}

impl IndexBuffer for GliumIndexBuffer {
    fn bind(&self) {}
    fn unbind(&self) {}
    fn get_count(&self) -> u32 { return self.count; }

    fn get_api(&self) -> API {
        return API::GLIUM; 
    }

    CASTIMPL!();
}

impl GliumIndexBuffer {
    pub fn new(ctx: SPtr<glium::backend::Context>, data: &[u32]) -> GliumIndexBuffer {
        return GliumIndexBuffer {
            count: data.len() as u32,
            buffer: glium::IndexBuffer::new(&ctx, glium::index::PrimitiveType::TrianglesList, data).expect("Failed To Init Glium Index Buffer")
        };
    }
}
