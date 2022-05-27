use gtk::prelude::*;
use gtk::{ApplicationWindow};

#[macro_use]
extern crate tarator;

mod glium_gl_area;
use glium_gl_area::GliumGLArea;

APPLICATION_DECLARE!(TarEd);
impl Application for TarEd {
    APPLICATION_LAYERIMPL!(TarEd);
    fn new() -> TarEd {

        // Load GL pointers from epoxy (GL context management library used by GTK).
        {
            #[cfg(target_os = "macos")]
            let library = unsafe { libloading::os::unix::Library::new("libepoxy.0.dylib") }.unwrap();
            #[cfg(all(unix, not(target_os = "macos")))]
            let library = unsafe { libloading::os::unix::Library::new("libepoxy.so.0") }.unwrap();
            #[cfg(windows)]
            let library = libloading::os::windows::Library::open_already_loaded("epoxy-0.dll").unwrap();

            epoxy::load_with(|name| {
                unsafe { library.get::<_>(name.as_bytes()) }
                    .map(|symbol| *symbol)
                    .unwrap_or(std::ptr::null())
            });
        }

        // Create Application
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
    
    let widget = GliumGLArea::new();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tarator Editor")
        .child(&widget)
        .build();

    // Present the window
    window.present();
}
