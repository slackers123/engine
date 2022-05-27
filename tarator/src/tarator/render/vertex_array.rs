use crate::{
    render::buffer::*,
    core::SPtr
};

/// ## VertexArray
/// all in one Vertex and Index Buffer
/// implemented in platform/*
pub trait VertexArray {
    fn new() -> Self where Self: Sized;

    fn bind();
    fn unbind();

    fn push_vertex_buffer(&mut self, buffer: &SPtr<dyn VertexBuffer>);
    fn set_index_buffer(&mut self, buffer: &SPtr<dyn IndexBuffer>);
}
