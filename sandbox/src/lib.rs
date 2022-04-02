use tarator::tarator::application::Application;

struct Sandbox {

}

impl Application for Sandbox {
    fn create_application() -> &'static Self { return &Sandbox{}; }
}