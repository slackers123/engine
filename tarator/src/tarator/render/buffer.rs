//! # Buffer
//! Here defined are the:
//! - ShaderDataType
//! - BufferElement
//! - BufferLayout
//! - VertexBuffer
//! - IndexBuffer
//! VertexArray consist of both VertexBuffer and IndexBuffer 

/// ## ShaderDataType
/// Enum of different Shader data types used in shaders
#[derive(Clone, Copy)]
#[allow(unused)]
enum ShaderDataType {
    None = 0,
    Float, Float2, Float3, Float4,
    Mat2, Mat3, Mat4,
    Int, Int2, Int3, Int4,
    Bool 
}

/// ## shader_data_type_size
/// Get the byte-size of ShaderDataType in u32
#[allow(warnings)]
fn shader_data_type_size(shader_data_type: ShaderDataType) -> u32 {
    match shader_data_type {
        ShaderDataType::None    => return  0,
        ShaderDataType::Float   => return  4,
        ShaderDataType::Float2  => return  8,
        ShaderDataType::Float3  => return 12,
        ShaderDataType::Float4  => return 16,
        ShaderDataType::Mat2    => return 16,
        ShaderDataType::Mat3    => return 36,
        ShaderDataType::Mat4    => return 64,
        ShaderDataType::Int     => return  4,
        ShaderDataType::Int2    => return  8,
        ShaderDataType::Int3    => return 12,
        ShaderDataType::Int4    => return 16,
        ShaderDataType::Bool    => return  1
    }

    TR_ERROR!("shader_data_type_size: Unknown ShaderDataType!");
    return 0;
}

#[derive(Clone, Copy)]
struct BufferElement {
    #[allow(unused)]
    name: &'static str,
    shader_data_type: ShaderDataType,
    size: u32,
    offset: u32,
    #[allow(unused)]
    normalized: bool
}

impl BufferElement {
    #[allow(warnings)]
    fn get_component_count(&self) -> u32 {
        match self.shader_data_type {
            ShaderDataType::None    => return  0,
            ShaderDataType::Float   => return  1,
            ShaderDataType::Float2  => return  2,
            ShaderDataType::Float3  => return  3,
            ShaderDataType::Float4  => return  4,
            ShaderDataType::Mat2    => return  4,
            ShaderDataType::Mat3    => return  9,
            ShaderDataType::Mat4    => return 16,
            ShaderDataType::Int     => return  1,
            ShaderDataType::Int2    => return  2,
            ShaderDataType::Int3    => return  3,
            ShaderDataType::Int4    => return  4,
            ShaderDataType::Bool    => return  1
        }

        TR_ERROR!("get_component_count: Unknown ShaderDataType!");
        return 0;
    }
}

pub struct BufferLayout {
    #[allow(unused)]
    elements: Vec<BufferElement>,
    #[allow(unused)]
    stride: u32
}

impl BufferLayout {
    #[allow(unused)]
    fn new(element: Vec<BufferElement>) -> BufferLayout {
        let mut buffer_layout: BufferLayout = BufferLayout { elements: element, stride: 0 };
        buffer_layout.calculate_offsets_and_stride();
        return buffer_layout;
    }
    #[allow(unused)]
    fn get_stride(&self) -> u32 { return self.stride; }
    #[allow(unused)]
    fn get_elements(&self) -> Vec<BufferElement> { return self.elements.clone();  }
    fn calculate_offsets_and_stride(&mut self) {
        let mut offset: u32 = 0;
        self.stride = 0;
        for element in &mut self.elements {
            element.offset = offset;
            offset += element.size;
            self.stride += element.size;
        }
    }
}

/// ## VertexBuffer
/// A buffer with vertices, used in VertexArray
/// implemented in platform/*
pub trait VertexBuffer {
    fn bind(&self);
    fn unbind(&self);
    fn get_layout(&self) -> BufferLayout;
    fn set_layout(&self, layout: &BufferLayout);
    fn new(vertices: &f32, size: u32) -> Self;
}

/// ## IndexBuffer
/// A buffer with indices, used in VertexArray
/// implemented in platform/*
pub trait IndexBuffer {
    fn bind(&self);
    fn unbind(&self);
    fn get_count(&self) -> u32;
    fn new(indices: &u32, size: u32) -> Self;
}
