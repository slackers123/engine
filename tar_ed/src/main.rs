use gtk::prelude::*;
use gtk::{ApplicationWindow, Orientation};

#[macro_use]
extern crate tarator;

APPLICATION_DECLARE!(TarEd);
impl Application for TarEd {
    APPLICATION_LAYERIMPL!(TarEd);
    fn new() -> TarEd {
        return TarEd {
            layer_stack: UPtr::new(LayerStack::new())
        }
    }
    fn run(&mut self) {
        // Create a new application
        let app = gtk::Application::builder()
        .application_id("Tarator Editor")
        .build();

        // Connect to "activate" signal of `app`
        app.connect_activate(build_ui);
            
        // Run the application
        app.run();
    }
}

fn build_ui(app: &gtk::Application) {
    


    let gtk_gl = gtk::GLArea::builder()
        .build();

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&gtk_gl);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tarator Editor")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
