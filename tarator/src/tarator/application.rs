pub trait Application {
    fn run() { loop {}; }
    fn create_application() -> &'static Self;
}