mod log_level;
mod logger;
mod pybindings;

use pyo3::prelude::*;

#[pymodule]
fn py_logkeeper(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<crate::pybindings::LogKeeper>()?;
    Ok(())
}
