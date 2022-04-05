pub trait Application {
    fn run(&self);
    fn create_application() -> &'static Self;
}