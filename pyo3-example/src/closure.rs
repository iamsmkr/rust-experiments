use pyo3::prelude::*;

#[pyfunction]
pub fn invoke_passed_func(py: Python, func: PyObject) -> PyResult<()> {
    let name = "Pometry";
    let args = (name,);
    func.call1(py, args)?;
    Ok(())
}

type O = Box<dyn Fn(&str) -> ()>;

#[pyfunction]
pub fn invoke_passed_func3(func: PyObject) -> PyResult<()> {
    let fun: O = Box::new(move |name| {
        Python::with_gil(|py| {
            func.call1(py, (name,)).unwrap();
        });
    });

    call_greet(fun);

    Ok(())
}

fn call_greet<F>(f: F)
where
    F: Fn(&str) -> (),
{
    let name = "Raphtory";
    f(name)
}
