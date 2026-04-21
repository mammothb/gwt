use log::{Level, LevelFilter, Log, Record};
use std::io::{stderr, stdout};

pub struct Logger {
    level: LevelFilter,
}

impl Logger {
    pub const fn new(level: LevelFilter) -> Self {
        Self { level }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn flush(&self) {
        use std::io::Write;
        let _ = stderr().flush();
        let _ = stdout().flush();
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        match record.level() {
            Level::Error => {
                eprintln!("{} - {}", record.level(), record.args());
            }
            _ => {
                println!("{} - {}", record.level(), record.args());
            }
        }
    }
}
