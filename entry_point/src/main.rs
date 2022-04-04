#[macro_use]
extern crate tarator;

use tarator::tarator::application::Application;
use sandbox::SandboxApplication;

fn main() {
    TR_LOG_INIT!();
    TR_ERROR!("Initialized Log!\n");

    let application = SandboxApplication::create_application();
    application.run();
}