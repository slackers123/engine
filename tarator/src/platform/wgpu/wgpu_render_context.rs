use crate::tarator::{window::Window, render::render_context::RenderContext};

/// ## WGPU Render Context
/// WGPU Implementation of tarator/render/render_context
pub struct WGPURenderContext<TWindow> where
    TWindow: Window {
    window: &TWindow
}

impl<TWindow> RenderContext for WGPURenderContext<TWindow> where
    TWindow: Window {
    fn new(window: &TWindow) -> WGPURenderContext {
        return WGPURenderContext<TWindow>{window};
    }
    fn init(&self) {
        
    }
}
