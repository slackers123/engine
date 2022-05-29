use glium::Surface;

use crate::{
    render::{
        vertex_array::*,
        *
    },
    core::*,
    math::*,
    platform::glium::{
        vertex_array::*,
        buffer::*
    }
};

pub struct GliumRenderAPI {
    pub ctx: SPtr<glium::backend::Context>,
    program: glium::Program    
}

impl RenderApi for GliumRenderAPI {
    fn init(&self) {
    }
    fn set_clear_color(&self, color: &F32Vec4) {
    }
    fn clear(&self) {
    }
    fn draw_indexed(&self, data: SPtr<dyn VertexArray>) {

        let vertex_array = match data.get_api() {
            API::GLIUM => CAST!(data, GliumVertexArray),
            _ => {
                TR_ERROR!("Wrong Vertex Array Implementation Provided");
                return;
            }
        };

        let vertex_buffer = vertex_array.get_vertex_buffer_iter();

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1f32]
            ]
        };

        let mut frame = glium::Frame::new(self.ctx.clone(), self.ctx.get_framebuffer_dimensions());
        frame.clear_color(0.8, 0.6, 0.1, 1.0);

        for buffer in vertex_array.get_vertex_buffer_iter() {
            frame.draw(
                &(CAST!(buffer, GliumVertexBuffer).buffer),
                &(CAST!(vertex_array.index, GliumIndexBuffer).buffer),
                &self.program,
                &uniforms,
&Default::default()
            ).unwrap();
        };
        frame.finish();
    }

    fn get_api() -> API {
        return API::GLIUM;
    }
}

impl GliumRenderAPI {
    pub fn new(ctx: SPtr<glium::backend::Context>) -> GliumRenderAPI {
        let program = program!(&ctx,
            140 => {
                vertex: "
                    #version 140
                    in vec2 position;
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                    }
                ",
                fragment: "
                    #version 140
                    out vec4 color;
                    void main() {
                        color = vec4(1.0, 0.0, 0.0, 1.0);
                    }
                "
            },
        )
        .unwrap();
        return GliumRenderAPI {
            ctx,
            program
        };
    }
}
