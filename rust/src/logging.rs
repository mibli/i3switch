use log;
use log::LevelFilter;
use simplelog::{ConfigBuilder, SimpleLogger};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        let config = ConfigBuilder::new()
            .set_location_level(LevelFilter::Trace)
            .set_target_level(LevelFilter::Off)
            .set_thread_level(LevelFilter::Off)
            .set_time_level(LevelFilter::Off)
            .add_filter_ignore_str("hyper") // Example filter for noisy crates
            .build();

        // Set log level based on build type
        let log_level = if cfg!(debug_assertions) {
            LevelFilter::Trace // Include all logs in debug builds
        } else {
            LevelFilter::Info // Exclude Trace and Debug in release builds
        };

        SimpleLogger::init(log_level, config).expect("Failed to initialize logger");
    });
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

// Custom macro to include file and function in logs
#[macro_export]
macro_rules! log_with_context {
    ($level:expr, $message:expr) => {{
        log::log!($level, "{} at {}:{}:{} ({})", $message, file!(), line!(), column!(), $crate::logging::function_name!());
    }};
}

macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log_with_context!(log::Level::Debug, format!($($arg)*));
    };
}

macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log_with_context!(log::Level::Info, format!($($arg)*));
    };
}

macro_rules! warning {
    ($($arg:tt)*) => {
        $crate::log_with_context!(log::Level::Warn, format!($($arg)*));
    };
}

macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log_with_context!(log::Level::Error, format!($($arg)*));
    };
}

macro_rules! critical {
    ($($arg:tt)*) => {
        $crate::log_with_context!(log::Level::Error, format!($($arg)*));
        panic!("Critical error encountered, terminating program.");
    };
}

pub trait ResultExt<T, E> {
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
                error!("{}: {}", msg, e);
                panic!("{}", msg);
            }
        }
    }
}

pub trait OptionExt<T> {
    fn wanted(self, msg: &str) -> Option<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn wanted(self, msg: &str) -> Option<T> {
        match self {
            Some(value) => Some(value),
            None => {
                warning!("{}", msg);
                None
            }
        }
    }
}

pub (super) use info;
pub (super) use warning;
pub (super) use error;
pub (super) use debug;
pub (super) use critical;
pub (super) use function_name;
