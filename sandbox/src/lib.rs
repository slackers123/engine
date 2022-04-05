extern crate tarator;
use tarator::tarator::application::Application;

pub struct SandboxApplication {

}

impl Application for SandboxApplication {
    fn run(&self) {
        loop {
            return;
        }
    }
    fn create_application() -> &'static Self { return &SandboxApplication{}; }
}