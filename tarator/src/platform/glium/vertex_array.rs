use crate::{
    render::{
        vertex_array::*,
        buffer::*,
        *
    },
    core::*,
    platform::glium::{
        buffer::*,
        render::*
    }
};

pub struct GliumVertexArray {
    vertex: Vector<SPtr<dyn VertexBuffer>>,
    pub index: SPtr<dyn IndexBuffer>
}

impl VertexArray for GliumVertexArray {
    fn bind(&self) {}
    fn unbind(&self) {}
    fn push_vertex_buffer(&mut self, buffer: SPtr<dyn VertexBuffer>) {
        
        self.vertex.push(buffer);
    }
    fn set_index_buffer(&mut self, buffer: SPtr<dyn IndexBuffer>) {
        self.index = buffer;
    }
    
    
    fn get_vertex_buffer_iter(&self) -> std::slice::Iter<SPtr<dyn VertexBuffer>> {
        return self.vertex.iter();
    }
    
    fn get_api(&self) -> API {
        return API::GLIUM;
    }
    
    CASTIMPL!();
}

impl GliumVertexArray {
    pub fn new(ctx: &SPtr<glium::backend::Context>) -> GliumVertexArray where GliumVertexArray: Sized  {
        return GliumVertexArray {
            vertex: Vector::new(),
            index: SPtr::new(GliumIndexBuffer::new(ctx, &[0]))
        };
    }
}
