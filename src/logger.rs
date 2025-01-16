//! Establishes a logger for the module.

use log::{Level, Metadata, Record};
use std::{fmt::Display, ptr::addr_of};

#[allow(dead_code)]
enum Color {
    Red,
    Yellow,
    Blue,
    Blank,
}
impl Color {
    const fn get_color_num(&self) -> &str {
        match self {
            Color::Red => "31",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Blank => "0",
        }
    }
}
// this could be a struct with an enum field
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\x1b[")?;
        f.write_str(self.get_color_num())?;
        f.write_str("m")
    }
}

#[derive(Debug)]
struct Logger {
    max_level: Level,
}
impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        match record.level() {
            Level::Error => eprintln!(
                "{red}Error{blank}: {args}",
                red = Color::Red,
                blank = Color::Blank,
                args = record.args()
            ),
            Level::Warn => eprintln!(
                "{yellow}Warning{blank}: {args}",
                yellow = Color::Yellow,
                blank = Color::Blank,
                args = record.args()
            ),
            // Level::Info => eprintln!(
            //     "{blue}Info{blank}: {args}",
            //     blue = Color::Blue,
            //     blank = Color::Blank,
            //     args = record.args()
            // ),
            Level::Info => eprintln!("{}", record.args()),
            Level::Debug => todo!(),
            Level::Trace => todo!(),
        }
    }

    fn flush(&self) {}
}

const DEFAULT_MAX_LOG: Level = Level::Info;
static mut LOGGER: Logger = Logger {
    max_level: DEFAULT_MAX_LOG,
};

/// Set global log level.
///
/// # Safety
/// Unsafe because mutates a static variable. Don't overuse it.
pub unsafe fn set_log_level(max_level: Level) {
    unsafe { LOGGER.max_level = max_level };
    log::set_max_level(max_level.to_level_filter());
}

/// Initialise the logger. Must be called before logger is used.
pub fn init_logger() {
    log::set_logger(unsafe { &*addr_of!(LOGGER) }).unwrap();
    log::set_max_level(DEFAULT_MAX_LOG.to_level_filter());
}
