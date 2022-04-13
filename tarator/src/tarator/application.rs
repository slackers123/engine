/// # Application
/// Here, the application base trait is implemented
/// Use this trait in another crate to create your application

use crate::tarator::window::{WindowProps, Window};

pub trait Application<TWindow> where
    TWindow: Window{
    fn new(window_props: &WindowProps) -> Self;
    fn run(&mut self);
}