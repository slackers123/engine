#[macro_use]
extern crate tarator;

use tarator::{
    tarator::{
        application::Application,
        window::{WindowProps}
    },
    platform::winit::winit_window::WinitWindow
};
use sandbox::SandboxApplication;

fn main() {
    TR_LOG_INIT!();
    TR_INFO!("Initialized Log!\n");

    let window_props: WindowProps = WindowProps { title: "Tarator Engine", width: 1280, height: 720 };
    let application: SandboxApplication<WinitWindow> = SandboxApplication::new(&window_props);
    application.run();
}