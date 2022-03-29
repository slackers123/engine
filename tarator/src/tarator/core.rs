#[macro_export]
macro_rules! BIT {
    ($arg:expr) => {
        (1 << $arg)
    };
}