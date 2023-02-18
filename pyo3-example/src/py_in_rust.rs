use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[pyfunction]
pub fn test_py_in_rust() -> PyResult<()> {
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
            "",
            "",
        )?
        .getattr("example")?
        .into();

        // Call object without any arguments
        fun.call0(py)?;

        // Call object with PyTuple
        let args = PyTuple::new(py, &[arg1, arg2, arg3]);
        fun.call1(py, args);

        // Pass arguments as rust tuple
        let args = (arg1, arg2, arg3);
        fun.call1(py, args)?;

        Ok(())
    })
}
