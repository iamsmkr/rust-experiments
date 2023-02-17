use pyo3::prelude::*;

#[pyclass]
pub struct VertexIterator {
    iter: Box<dyn Iterator<Item = u64> + Send>,
}

#[pymethods]
impl VertexIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<u64> {
        slf.iter.next()
    }
}

#[pyfunction]
pub fn get_vertices() -> VertexIterator {
    let i = vec![1u64, 2, 3, 4, 5].into_iter();
    VertexIterator { iter: Box::new(i) }
}

#[pyclass]
pub struct ItemIterator {
    iter: std::vec::IntoIter<u64>,
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

#[pyclass]
pub struct Warehouse {
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

    fn get_items(&self) -> ItemIterator {
        ItemIterator {
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

#[pyclass]
pub struct PersonIterator {
    iter: std::vec::IntoIter<Person>,
}

#[pymethods]
impl PersonIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Person> {
        slf.iter.next()
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Person {
    #[pyo3(get)]
    pub id: u64,
}

impl Person {
    fn new(id: u64) -> Person {
        Person { id }
    }
}

#[pyclass]
pub struct People {
    people: Vec<Person>,
}

#[pymethods]
impl People {
    #[new]
    fn new() -> People {
        People {
            people: vec![Person::new(5), Person::new(3)],
        }
    }

    fn get_people(&self) -> PersonIterator {
        PersonIterator {
            iter: self
                .people
                .iter()
                .map(|f| *f)
                .collect::<Vec<Person>>()
                .into_iter(),
        }
    }
}
