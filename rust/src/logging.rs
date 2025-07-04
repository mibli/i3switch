/// A simple logging module for Rust with custom macros and context information.
/// Logging follows the format inspired by glibc based desktop applications,
/// especially i3 window manager.
///
/// The choice is for a consistent logging style in the environment.
///
/// For example, a debug log would look like:
/// i3switch: DEBUG: file.rs:123: Entering function

#[derive(Debug)]
pub enum Level {
    DEBUG,
    INFO,
    WARNING,
    ERROR,
}

#[macro_export]
macro_rules! log {
    ($level:expr, $message:expr) => {{
        #[cfg(debug_assertions)]
        println!("i3switch: [{:?}] {}:{}: {}", $level, file!(), line!(), $message);
        #[cfg(not(debug_assertions))]
        println!("i3switch: {:?}: {}", $level, $message);
    }};
}

#[macro_export]
macro_rules! elog {
    ($level:expr, $message:expr) => {{
        #[cfg(debug_assertions)]
        eprintln!("i3switch: [{:?}] {}:{}: {}", $level, file!(), line!(), $message);
        #[cfg(not(debug_assertions))]
        eprintln!("i3switch: {:?}: {}", $level, $message);
    }};
}

macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logging::log!($crate::logging::Level::DEBUG, format!($($arg)*));
    };
}

macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logging::log!($crate::logging::Level::INFO, format!($($arg)*));
    };
}

macro_rules! warning {
    ($($arg:tt)*) => {
        $crate::logging::log!($crate::logging::Level::WARNING, format!($($arg)*));
    };
}

macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logging::elog!($crate::logging::Level::ERROR, format!($($arg)*));
    };
}

macro_rules! critical {
    ($($arg:tt)*) => {
        $crate::logging::elog!($crate::logging::Level::ERROR, format!($($arg)*));
        panic!("Critical error encountered, terminating program.");
    };
}

pub trait ResultExt<T, E> {
    #[allow(dead_code)]
    fn expect_log(self, msg: &str) -> T;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn expect_log(self, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                crate::logging::error!("{}: {}", msg, e);
                panic!("{}", msg);
            }
        }
    }
}

pub trait OptionExt<T> {
    #[allow(dead_code)]
    fn wanted(self, msg: &str) -> Option<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn wanted(self, msg: &str) -> Option<T> {
        match self {
            Some(value) => Some(value),
            None => {
                crate::logging::warning!("{}", msg);
                None
            }
        }
    }
}

pub (super) use log;
pub (super) use elog;
pub (super) use info;
pub (super) use warning;
pub (super) use error;
pub (super) use debug;
pub (super) use critical;
