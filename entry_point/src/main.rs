#[macro_use] extern crate tarator;
use tarator::tarator::application::Application;
use sandbox::SandboxApplication;
fn main() {
    TR_LOG_INIT!();
    TR_INFO!("Initialized Log!\n");
    let mut application = SandboxApplication::new();
    application.run();
}
