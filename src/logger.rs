//! Establishes a logger for the module.

use std::fmt::Display;

use log::{Level, LevelFilter, Metadata, Record};

enum Color {
    Red,
    Blank,
}
impl Color {
    const fn get_color_num(&self) -> &str {
        match self {
            Color::Red => "31",
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

struct SimpleExampleLogger;
impl log::Log for SimpleExampleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
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
            Level::Warn => todo!(),
            Level::Info => todo!(),
            Level::Debug => todo!(),
            Level::Trace => todo!(),
        }
        println!("{} - {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

static LOGGER: SimpleExampleLogger = SimpleExampleLogger;

/// Initialise the logger. Must be called before logger is used.
pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
}
