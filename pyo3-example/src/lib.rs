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
fn get_numbers() -> ItemIterator {
    let i = vec![1u64, 2, 3, 4, 5].into_iter();
    ItemIterator { iter: Box::new(i) }
}

#[pyclass]
struct ItemIterator2 {
    iter: std::vec::IntoIter<u64>,
}

#[pymethods]
impl ItemIterator2 {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<u64> {
        slf.iter.next()
    }
}

#[pyclass]
struct Warehouse {
    items: Vec<u64>,
}

#[pymethods]
impl Warehouse {
    #[new]
    fn new() -> Warehouse {
        Warehouse {
            items: vec![1u64, 2, 3, 4, 5],
        }
    }

    fn get_items(&self) -> ItemIterator2 {
        ItemIterator2 {
            iter: self
                .items
                .iter()
                .map(|f| *f)
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

// fn get_items(&self) -> Box<dyn Iterator<Item = u64> + '_> {
//     Box::new(self.items.iter().map(|f| *f))
// }

#[pymodule]
fn pyo3_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_numbers, m)?)?;
    m.add_class::<ItemIterator>()?;
    m.add_class::<ItemIterator2>()?;
    m.add_class::<Warehouse>()?;
    Ok(())
}
