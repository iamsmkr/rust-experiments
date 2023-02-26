use std::collections::HashMap;

use polars::prelude::*;
use pyo3::{pyclass, pyfunction, pymethods, types::IntoPyDict, FromPyObject, PyResult};
use serde::{Deserialize, Serialize};

// {
//     "src": {
//         col: 1,
//         t: 3,
//         prop: [("type": "character")]
//     },
//     "dst": {
//         col: 2,
//         t: 3,
//         prop: [("type": "co-character")]
//     }
// }

#[derive(FromPyObject, Debug, Clone)]
pub enum Prop {
    Int(usize),
    String(String),
    Vec(Vec<usize>),
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Vertex {
    col: i32,
    t: i64,
    props: HashMap<String, Prop>,
}

#[pymethods]
impl Vertex {
    #[new]
    fn new(col: i32, t: i64, props: HashMap<String, Prop>) -> Self {
        Vertex { col, t, props }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Schema {
    src: Vertex,
    dst: Vertex,
}

#[pymethods]
impl Schema {
    #[new]
    fn new(src: Vertex, dst: Vertex) -> Self {
        Schema { src, dst }
    }
}

#[pyfunction]
pub fn read_lotr(schema: Schema) -> PyResult<()> {
    let mut df = CsvReader::from_path("data/lotr.csv")
        .unwrap()
        .has_header(false)
        .finish()
        .unwrap();

    assert!(!df.is_empty());

    println!("{:?}", df);

    df.as_single_chunk_par();
    let mut iters = df
        .columns(["column_1", "column_2", "column_3"])
        .unwrap()
        .iter()
        .map(|s| s.iter())
        .collect::<Vec<_>>();

    for row in 0..df.height() {
        for iter in &mut iters {
            let value = iter.next().expect("should have as many iterations as rows");
            println!("{value}")
        }
    }

    Ok(())
}
