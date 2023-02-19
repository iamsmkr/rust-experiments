use pyo3::prelude::*;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct Prop;

#[pymethods]
impl Prop {
    #[new]
    fn new() -> Self {
        Prop
    }

    pub fn method(&self) {}
}

#[pyclass(extends=Prop, subclass)]
#[derive(Clone)]
pub struct Str {
    value: String,
}

#[pymethods]
impl Str {
    #[new]
    fn new(value: String) -> (Self, Prop) {
        (Str { value }, Prop::new())
    }

    pub fn method(&self) {
        println!("value = {}", self.value)
    }
}

#[pyclass(extends=Prop, subclass)]
#[derive(Clone)]
pub struct Int {
    value: usize,
}

#[pymethods]
impl Int {
    #[new]
    fn new(value: usize) -> (Self, Prop) {
        (Int { value }, Prop::new())
    }

    pub fn method(&self) {
        println!("value = {}", self.value)
    }
}

// This doesn't work! Refer: https://github.com/PyO3/pyo3/discussions/2959#discussioncomment-5000973
// #[pyfunction]
// fn print_prop(s: Prop) {
//     s.method()
// }

#[pyfunction]
pub fn print_prop(s: &PyCell<Prop>) -> PyResult<&PyAny> {
    s.call_method0("method")
}

#[pyfunction]
pub fn print_str(s: Str) {
    println!("{}", s.value)
}

// Create a struct that any python class can extend
#[pyclass(subclass)]
pub struct Greeter {
    name: String,
}

#[pymethods]
impl Greeter {
    #[new]
    fn new(name: String) -> Self {
        Self { name }
    }

    fn say_hello(&self) {
        println!("Hello {}", self.name);
    }
}
