use crate::{
    render::{
        buffer::*,
        *
    },
    core::SPtr
};

/// ## VertexArray
/// all in one Vertex and Index Buffer
/// implemented in platform/*
pub trait VertexArray {
    fn bind(&self);
    fn unbind(&self);

    fn push_vertex_buffer(&mut self, buffer: SPtr<dyn VertexBuffer>);
    fn set_index_buffer(&mut self, buffer: SPtr<dyn IndexBuffer>);
    
    fn get_vertex_buffer_iter(&self) -> std::slice::Iter<SPtr<dyn VertexBuffer>>;

    fn get_api(&self) -> API;
    
    CASTIMPLTRAIT!();
}
