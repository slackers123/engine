extern crate glfw;

#[allow(unused)]
use crate::tarator::{
    core::{UPtr, SPtr},
    render::render_context::RenderContext,
    window::{
        EventCallbackFn, Null,
        Window,
        WindowProps
    },
    event::{
        Event,
        application_event::*
    }
};
mod g {
    pub use glfw::{
        Window,
        ffi::*
    };
}
use std::os::raw::*;
use std::ffi::CString;
/// GLFW UNSPECIFIC DATA
static mut GLFWINITIALIZED: bool = false;
extern "C" fn glfw_error_callback(error: i32, description: *const c_char) {
    unsafe {
        TR_ERROR!("GLFW ERROR ({}): {}", error, description.as_ref().expect("NOT UTF8!"));
    }
}
/// GLFW UNSPECIFIC DATA
/// ## GLFWWindowData
struct GLFWWindowData {
    #[allow(unused)]
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
    #[allow(unused)]
    event_callback: EventCallbackFn
}
/// ## GLFWWindow
pub struct GLFWWindow {
    window: *mut g::Window,
    // context: &'a dyn RenderContext,
    data: GLFWWindowData
}
impl Window for GLFWWindow {
    fn update(&mut self) {}
    fn get_width(&self) -> u32 { return self.data.width; }
    fn get_height(&self) -> u32 { return self.data.height; }

    #[allow(unused)]
    fn set_event_callback(&self, callback: &EventCallbackFn) {}
    fn set_vsync(&mut self, enabled: bool) { self.data.vsync = enabled; }
    fn get_vsync_enabled(&self) -> bool { return self.data.vsync; }

    fn new(window_props: &WindowProps) -> GLFWWindow {
        unsafe {
            if !GLFWINITIALIZED {
                let success = g::glfwInit();
                TR_ASSERT!(success, "Couldn't Init GLFW!");
                g::glfwSetErrorCallback(Option::Some(glfw_error_callback));
                GLFWINITIALIZED = true;
            }
            let mut r_window = GLFWWindow {
                window: g::glfwCreateWindow(
                        window_props.width as i32,
                        window_props.height as i32,
                        #[allow(temporary_cstring_as_ptr)]
                        CString::new(window_props.title.clone()).expect("CSting Failed To Create").as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut()
                    ) as *mut g::Window,
                // context: 0,
                data: GLFWWindowData {
                    title: window_props.title.clone(),
                    width: window_props.width,
                    height: window_props.height,
                    vsync: true,
                    event_callback: EventCallbackFn::null()
                }
            };
            g::glfwSetWindowUserPointer(
                r_window.window as *mut g::GLFWwindow,
                &mut r_window.data as *mut _ as *mut c_void
            );
            r_window.set_vsync(true);
            // EVENT SETUP:
            // g::glfwSetWindowCloseCallback(
            //     r_window.window  as *mut g::GLFWwindow,
            //     Some(|window| {
            //         let data = g::glfwGetWindowUserPointer(window as *mut g::GLFWwindow) as *mut _ as *mut GLFWWindowData;
            //         let event: &dyn Event = &WindowCloseEvent::default();
            //         ((&*data).event_callback)(event);
            //     })
            // );
            return r_window;
        }
    }
}