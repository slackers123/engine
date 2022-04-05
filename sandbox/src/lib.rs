extern crate tarator;
use tarator::{tarator::application::Application, TR_DEBUG};

pub struct SandboxApplication {

}

impl Application for SandboxApplication {
    fn run(&self) {
        loop {
            TR_DEBUG!("Exiting...");
            return; 
        }
    }
    fn create_application() -> &'static Self { return &SandboxApplication{}; }
}