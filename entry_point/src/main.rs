#[macro_use]
extern crate tarator;

#[allow(unused)]
use tarator::{
    tarator::{
        application::Application,
        window::{WindowProps}
    },
    platform::{
        glfw::glfw_window::GLFWWindow,
        winit::winit_window::WinitWindow
    }
};
use sandbox::SandboxApplication;

fn main() {
    TR_LOG_INIT!();
    TR_INFO!("Initialized Log!\n");

    //  Currently To Change Window API you have to Replace Te Generic Type Here:
    //                                     \\         //
    let mut application: SandboxApplication<GLFWWindow> = SandboxApplication::new(
        &WindowProps { title: String::from("Tarator Engine"), width: 1280, height: 720 }
    );
    application.run();
}
