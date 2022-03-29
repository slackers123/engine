/// # log.rs
/// Collection of macros for debugging purposes
/// TODO: add more macros
///       redesign that whole baby I guess

#[macro_export]
macro_rules! TR_ERROR {
    ($($arg:tt)*) => {
        log::error!($($arg)*);
    };
}