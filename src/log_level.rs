use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl LogLevel {
    pub fn value(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
            LogLevel::Critical => "CRITICAL",
        }
    }
}

// Implementa Display para poder usar {} en format!
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
