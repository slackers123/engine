#[allow(unused)]
#[macro_use]
extern crate tarator;
use tarator::{tarator::{
    application::Application,
    window::{WindowProps, Window},
    core::UPtr,
    event::*
}};

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
    fn run(&mut self) {
        loop {
            let event = self.window.update();

            if event.get_action() == EventAction::WINDOWCLOSE {
                break;
            }
        }
    }
}
