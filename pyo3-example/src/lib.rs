use pyo3::prelude::*;

#[pyclass]
struct VertexIterator {
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
fn get_vertices() -> VertexIterator {
    let i = vec![1u64, 2, 3, 4, 5].into_iter();
    VertexIterator { iter: Box::new(i) }
}

#[pyclass]
struct ItemIterator {
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
struct PersonIterator {
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
struct People {
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

    fn getPeople(&self) -> PersonIterator {
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

#[pyclass(subclass)]
#[derive(Clone)]
struct BaseClass;

#[pymethods]
impl BaseClass {
    #[new]
    fn new() -> Self {
        BaseClass
    }
}

impl BaseClass {}

#[pyclass(extends=BaseClass, subclass)]
#[derive(Clone)]
struct SubClass {
    value: usize,
}

#[pymethods]
impl SubClass {
    #[new]
    fn new(value: usize) -> (Self, BaseClass) {
        (SubClass { value }, BaseClass::new())
    }
}

#[pyfunction]
fn takes_args(s: SubClass) {
    println!("{}", s.value);
}

#[pymodule]
fn pyo3_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_vertices, m)?)?;
    m.add_function(wrap_pyfunction!(takes_args, m)?)?;
    m.add_class::<VertexIterator>()?;
    m.add_class::<ItemIterator>()?;
    m.add_class::<PersonIterator>()?;
    m.add_class::<Warehouse>()?;
    m.add_class::<Person>()?;
    m.add_class::<People>()?;
    m.add_class::<BaseClass>()?;
    m.add_class::<SubClass>()?;
    Ok(())
}
