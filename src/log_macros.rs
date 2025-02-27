#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        log::debug!($($arg)*);
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! trace_log {
    ($($arg:tt)*) => {
        log::trace!($($arg)*);
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! trace_log {
    ($($arg:tt)*) => {};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! info_log {
    ($($arg:tt)*) => {
        log::info!($($arg)*);
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! info_log {
    ($($arg:tt)*) => {};
}
