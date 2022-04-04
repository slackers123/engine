/// # log.rs
/// Collection of macros for debugging purposes
/// TODO: add more macros
///       redesign that whole baby I guess

#[macro_export]
macro_rules! TR_LOG_INIT {
    () => {
       env_logger::init(); 
    };
}
#[macro_export]
macro_rules! TR_ERROR {
    ($($arg:tt)*) => {
        log::error!($($arg)*);
    };
}
#[macro_export]
macro_rules! TR_WARN {
    ($($arg:tt)*) => {
        log::warn!($($arg)*);
    };
}
#[macro_export]
macro_rules! TR_INFO {
    ($($arg:tt)*) => {
        log::info!($($arg)*);
    };
}
#[macro_export]
macro_rules! TR_DEBUG {
    ($($arg:tt)*) => {
        log::debug!($($arg)*);
    };
}
#[macro_export]
macro_rules! TR_TRACE {
    ($($arg:tt)*) => {
        log::trace!($($arg)*);
    };
}