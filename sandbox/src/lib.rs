#[allow(unused)]
#[macro_use]
extern crate tarator;
use tarator::{tarator::{
    application::Application,
    window::{WindowProps, Window},
    core::{UPtr, SPtr},
    event::*,
    layer::{Layer, LayerStack}
}};

struct ExampleLayer {
    name: String
}
impl Layer for ExampleLayer {
    fn new(name: String) -> ExampleLayer {
        return ExampleLayer {
            name: name
        };
    }    
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

APPLICATION_DECLARE!(SandboxApplication);
impl<TWindow> Application<TWindow> for SandboxApplication<TWindow> where
    TWindow: Window { APPLICATION_DEFAULTIMPL!(SandboxApplication);
    // RUN
    fn new(window_props: &WindowProps) -> SandboxApplication<TWindow> {
        let mut app = SandboxApplication{
            window: UPtr::new(TWindow::new(window_props)),
            layer_stack: UPtr::new(LayerStack::new())
        };
        app.push_layer(SPtr::new(ExampleLayer::new(String::from("Example Layer"))));
        return app;
    }
}
