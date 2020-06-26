
use log::{Level, Log, Metadata, Record, SetLoggerError};
use alloc::string::String;
use alloc::boxed::Box;

use crate::println;


struct KernelLogger {
    level: Level,
}

impl Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_string = match record.level() {
                Level::Error => String::from("Error"),
                Level::Warn => String::from("Warn"),
                Level::Info => String::from("Info"),
                Level::Debug => String::from("Debug"),
                Level::Trace => String::from("Trace"),
            };
            let file = record.file().unwrap_or("unknownfile");
            let line = record.line().unwrap_or(0);
            println!("[{:<5} {}:{}] {}", level_string, file, line, record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    let logger = KernelLogger { level };
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&logger)?;
    Ok(())
}