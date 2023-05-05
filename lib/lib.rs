#![feature(exclusive_range_pattern)]

pub mod attribute_info;
pub mod attributes;
pub mod class;
pub mod constant_info;
pub mod constant_tags;
pub mod error;
pub mod field_info;
pub mod interpeter;
pub mod java_class;
pub mod jvm;
pub mod jvm_state;
pub mod method_info;
pub mod opcodes;
pub mod stream;
pub mod types;

#[cfg(debug_assertions)]
pub mod logger;

pub mod prelude {
    pub use crate::logger;
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! LogInit {
    () => {
        logger::set_logger(Some(logger::Logger::new()));
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! LogInit {
    () => {};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! Log {
    ($name: expr, $desc: expr, $value: expr) => {
        if let Some(logger) = logger::get_logger() {
            logger.add_action(logger::Action::new($name, $desc, $value));
        }
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! Log {
    ($name: expr, $desc: expr, $value: expr) => {};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! LogDone {
    () => {
        if let Some(logger) = logger::get_logger() {
            logger.exit_action();
        }
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! LogDone {
    () => {};
}
