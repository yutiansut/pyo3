use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug, PartialEq)]
struct Cloneable {
    x: i32,
}

#[test]
fn test_cloneable_pyclass() {
    let c = Cloneable { x: 10 };

    let gil = Python::acquire_gil();
    let py = gil.python();

    let py_c = Py::new(py, c.clone()).unwrap().to_object(py);

    let c2: Cloneable = py_c.extract(py).unwrap();
    let rc: &Cloneable = py_c.extract(py).unwrap();
    let mrc: &mut Cloneable = py_c.extract(py).unwrap();

    assert_eq!(c, c2);
    assert_eq!(&c, rc);
    assert_eq!(&c, mrc);
}
