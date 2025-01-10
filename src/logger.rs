use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

static LOGGER: SimpleExampleLogger = SimpleExampleLogger;

pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
}

struct SimpleExampleLogger;

impl log::Log for SimpleExampleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
