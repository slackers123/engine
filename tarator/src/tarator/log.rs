//! # Log
//! Collection of macros for debugging purposes
/// ## TR_LOG_INIT
/// Has to be called before calling any of the other logging macros!
#[macro_export]
macro_rules! TR_LOG_INIT {
    () => {
       env_logger::Builder::new().target(env_logger::Target::Stderr).init() 
    };
}
/// ## TR_ERROR
#[macro_export]
macro_rules! TR_ERROR {
    ($($arg:tt)*) => {
        log::error!($($arg)*)
    };
}
/// ## TR_WARN
#[macro_export]
macro_rules! TR_WARN {
    ($($arg:tt)*) => {
        log::warn!($($arg)*)
    };
}
/// ## TR_INFO
#[macro_export]
macro_rules! TR_INFO {
    ($($arg:tt)*) => {
        log::info!($($arg)*)
    };
}
/// ## TR_DEBUG
#[macro_export]
macro_rules! TR_DEBUG {
    ($($arg:tt)*) => {
        log::debug!($($arg)*)
    };
}
/// ## TR_TRACE
#[macro_export]
macro_rules! TR_TRACE {
    ($($arg:tt)*) => {
        log::trace!($($arg)*)
    };
}