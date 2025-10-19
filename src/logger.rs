use crate::log_level::LogLevel;
use chrono::Local;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{self, Write};
use std::path::PathBuf;

/// Internal logger structure (thread-safe)
pub struct InnerLogger {
    file: File,
}

impl InnerLogger {
    pub fn new(log_dir: &PathBuf) -> io::Result<Self> {
        create_dir_all(log_dir)?;
        let now = Local::now();
        let filename = now.format("%Y-%m-%d_%H-%M-%S.log").to_string();
        let mut path = log_dir.clone();
        path.push(filename);

        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Self { file })
    }

    pub fn write_entry(&mut self, level: LogLevel, message: &str) -> io::Result<()> {
        let ts = Local::now().format("%H:%M:%S").to_string();

        writeln!(self.file, "[{}] {}: {}", ts, level.value(), message)?;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

// Global singleton
static LOGGER: OnceCell<Mutex<InnerLogger>> = OnceCell::new();

pub fn get_logger() -> &'static Mutex<InnerLogger> {
    LOGGER.get_or_init(|| {
        let mut default_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        default_dir.push("logs");
        let inner = InnerLogger::new(&default_dir).expect("Failed to initialize LogKeeper");
        Mutex::new(inner)
    })
}
