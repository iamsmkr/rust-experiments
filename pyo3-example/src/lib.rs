use pyo3::prelude::*;

#[pyclass]
struct ItemIterator {
    iter: Box<dyn Iterator<Item = u64> + Send>,
}

#[pymethods]
impl ItemIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<u64> {
        slf.iter.next()
    }
}

#[pyfunction]
fn items() -> ItemIterator {
    let i = vec![1u64, 2, 3, 4, 5].into_iter();
    ItemIterator { iter: Box::new(i) }
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(items, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<ItemIterator>()?;
    Ok(())
}
