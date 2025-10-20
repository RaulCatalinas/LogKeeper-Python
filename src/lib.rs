mod log_level;
mod logger;
mod pybindings;

use pyo3::prelude::*;

#[pymodule]
fn logkeeper_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<crate::pybindings::LogKeeper>()?;
    Ok(())
}
