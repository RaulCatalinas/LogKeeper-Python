use crate::log_level::LogLevel;
use crate::logger::get_logger;
use pyo3::prelude::*;

#[pyclass]
pub struct LogKeeper {}

fn write_log(level: LogLevel, msg: &str) -> PyResult<()> {
    let mut logger = get_logger().lock();
    logger
        .write_entry(level, msg)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("{}", e)))
}

#[pymethods]
impl LogKeeper {
    #[staticmethod]
    pub fn info(msg: &str) -> PyResult<()> {
        write_log(LogLevel::Info, msg)
    }

    #[staticmethod]
    pub fn warning(msg: &str) -> PyResult<()> {
        write_log(LogLevel::Warning, msg)
    }

    #[staticmethod]
    pub fn error(msg: &str) -> PyResult<()> {
        write_log(LogLevel::Error, msg)
    }

    #[staticmethod]
    pub fn critical(msg: &str) -> PyResult<()> {
        write_log(LogLevel::Critical, msg)
    }

    #[staticmethod]
    pub fn save_logs() -> PyResult<()> {
        let mut logger = crate::logger::get_logger().lock();
        logger
            .flush()
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("{}", e)))
    }
}
