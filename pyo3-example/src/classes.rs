use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

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

// serde: Attempt 1
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[pyfunction]
pub fn point_serde() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

// Attempt 2
#[derive(Serialize, Deserialize, Debug)]
#[pyclass(subclass)]
pub struct General;

#[pymethods]
impl General {
    #[new]
    fn new() -> Self {
        Self
    }
}

#[pyfunction]
pub fn point_serde2(point: &PyCell<General>) -> PyResult<()> {
    let r: &PyAny = point.as_ref();
    println!("{:?}", point.as_ref());
    // Refer: https://github.com/PyO3/pyo3/discussions/2967

    // let serialized = serde_json::to_string(r).unwrap();
    // println!("serialized = {}", serialized);

    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    // println!("deserialized = {:?}", deserialized);

    // s.call_method0("method")

    Ok(())
}
