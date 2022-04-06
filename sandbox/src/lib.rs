#[allow(unused)]
#[macro_use]
extern crate tarator;
use tarator::{tarator::{application::Application, window::{WindowProps, Window}, core::UPtr}};

pub struct SandboxApplication<TWindow> where
    TWindow: Window {
    window: UPtr<TWindow>
}

impl<TWindow> Application<TWindow> for SandboxApplication<TWindow> where
    TWindow: Window 
{
    fn new(window_props: &WindowProps) -> Self {
        return SandboxApplication{
            window: UPtr::new(TWindow::new(window_props))
        };
    }
    fn run(&self) {
        loop {
            self.window.update();
        }
    }
}
