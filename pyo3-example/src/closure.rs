use pyo3::prelude::*;

#[pyfunction]
pub fn invoke_passed_func(func: PyObject) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<()> {
        let name = "Shivam";
        let args = (name,);
        func.call1(py, args)?;
        Ok(())
    })
}
