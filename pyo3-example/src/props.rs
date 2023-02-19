use pyo3::prelude::*;
use std::collections::HashMap;

#[derive(FromPyObject, Debug)]
pub enum Prop {
    Int(usize),
    String(String),
    Vec(Vec<usize>),
}

#[pyfunction]
pub fn get_props(props: HashMap<String, Prop>) -> PyResult<()> {
    let v = props.into_iter().collect::<Vec<(String, Prop)>>();
    for i in v {
        println!("K = {}, V = {:?}", i.0, i.1)
    }
    Ok(())
}
