use pyo3::prelude::*;

#[pyfunction]
pub fn invoke_passed_func(func: PyObject) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<()> {
        func.call0(py)?;
        Ok(())
    })
}
