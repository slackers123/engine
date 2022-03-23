use crate::renderer::{InitInfo, MainloopInfo};
use crate::renderers::renderer::Renders;

use glium::{glutin, implement_vertex, uniform};
use glium::Surface;
use glium::program::Program;
use glutin::event_loop::EventLoop;
use glium::backend::glutin::Display;


pub struct gl_renderer {
    program: Program,
    event_loop: EventLoop<()>,
    display: Display,
}

impl gl_renderer {
    pub fn new(info: InitInfo) -> Self {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

        let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        gl_renderer {
            program,
            event_loop,
            display,
        }
    }
}

impl Default for gl_renderer{
    fn default() -> Self {
        gl_renderer::new(InitInfo::default())
    }
}


impl Renders for gl_renderer {
    fn init(&self, info: InitInfo) {
        println!("TODO: deprecate this function")
    }

    fn main_loop(self, info: MainloopInfo) {

        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }

        implement_vertex!(Vertex, position);

        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ 0.0,  0.5] };
        let vertex3 = Vertex { position: [ 0.5, -0.25] };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        self.event_loop.run(move |event, _, control_flow| {
            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }

            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            target.draw(&vertex_buffer, &indices, &self.program, &glium::uniforms::EmptyUniforms,
                        &Default::default()).unwrap();
            target.finish().unwrap();
        });
    }
}