use log::{Level, Log, Metadata, Record, SetLoggerError};
use alloc::string::String;

use crate::println;

struct KernelLogger {
    level: Level,
}

impl Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
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
            let target = if record.target().len() > 0 {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };
            println!("{:<5} [{}] {}", level_string, target, record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    #[cfg(all(windows, feature = "colored"))]
    set_up_color_terminal();

    let logger = SimpleLogger { level };
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}