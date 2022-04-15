use crate::{tarator::{
    window::{WindowProps, Window},
    event::{
        *,
        application_event::*,
        key_event::*
    },
    core::{UPtr, SPtr}},
    platform::winit::winit_keycode::get_tr_keycode
};
mod w {
    pub use winit::{
        dpi::LogicalSize,
        window::WindowBuilder,
        event_loop::ControlFlow,
        event::{
            WindowEvent,
            KeyboardInput,
            MouseButton,
            MouseScrollDelta,
            Event,
            ElementState
        }
    };
}
use winit::platform::run_return::EventLoopExtRunReturn;
/// ## WinitWindowData
/// 
struct WinitWindowData/*<'a>*/ {
    #[allow(unused)]
    title: String,
    #[allow(unused)]
    width: u32,
    #[allow(unused)]
    height: u32,
    #[allow(unused)]
    vsync: bool
}
/// ## WGPU Implementation of window trait
/// tarator/window.rs
pub struct WinitWindow {
    #[allow(unused)]
    event_loop: UPtr<winit::event_loop::EventLoop<()>>,
    #[allow(unused)]
    window: UPtr<winit::window::Window>,
    #[allow(unused)]
    data: WinitWindowData
}
impl Window for WinitWindow {
    fn update(&mut self) -> SPtr<dyn Event> {
        let mut return_event: SPtr<dyn Event> = SPtr::new(ApplicationUpdateEvent::default());
        // get event from winit
        self.event_loop.run_return(|event, _target, control_flow| {
            *control_flow = w::ControlFlow::Exit; // we wan't to immediately exit so we can return a proper event every update loop cycle
            match event {
                w::Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                        match event {
                            w::WindowEvent::CloseRequested => {
                                return_event = SPtr::new(WindowCloseEvent::default());
                            },
                            w::WindowEvent::KeyboardInput {
                                input: w::KeyboardInput {
                                    state: w::ElementState::Pressed,
                                    virtual_keycode: Some(keycode),
                                    ..
                                },
                                ..
                            } => {
                                return_event = SPtr::new(KeyPressedEvent::new(get_tr_keycode(keycode)));
                            },
                            _ => {}
                        }
                },
                _ => {}
            };
        });
        return return_event;
    }
    fn get_width(&self) {}
    fn get_height(&self) {}

    #[allow(unused)]
    fn set_vsync(&mut self, enabled: bool) { self.data.vsync = enabled; }
    fn get_vsync_enabled(&self) -> bool { return self.data.vsync; }

    #[allow(unused)]
    fn new(window_props: &WindowProps) -> WinitWindow {
        TR_INFO!("Executing WinitWindow::new();\n");
        let event_loop= winit::event_loop::EventLoop::new();
        let data: WinitWindowData = WinitWindowData {
            title: window_props.title.clone(),
            width: window_props.width,
            height: window_props.height,
            vsync: true                  // VSYNC IS HARDCODED HERE
        };
        let window = match w::WindowBuilder::new()
                .with_inner_size(w::LogicalSize {
                    width: data.width,
                    height: data.height
                })
                .build(&event_loop) {
            Ok(window) => window,
            Err(os_error) => panic!("Failed To Create Window!")
        };
        return WinitWindow {
            event_loop: UPtr::new(event_loop),
            window: UPtr::new(window),
            data: data
        };
    }
}
