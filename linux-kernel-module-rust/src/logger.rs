use log::{Level, Log, Metadata, Record, SetLoggerError};
use alloc::string::String;

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
                Level::Error => String::from("Error").red(),
                Level::Warn => String::from("Warn").yellow(),
                Level::Info => String::from("Info").cyan(),
                Level::Debug => String::from("Debug").purple(),
                Level::Trace => String::from("Trace").normal(),
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
