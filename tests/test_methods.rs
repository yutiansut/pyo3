use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::types::{IntoPyDict, PyDict, PyList, PySet, PyString, PyTuple, PyType};
use pyo3::PyClassShell;

mod common;

#[pyclass]
struct InstanceMethod {
    member: i32,
}

#[pymethods]
impl InstanceMethod {
    /// Test method
    fn method(&self) -> PyResult<i32> {
        Ok(self.member)
    }
}

#[test]
fn instance_method() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let obj = PyClassShell::new_mut(py, InstanceMethod { member: 42 }).unwrap();
    assert_eq!(obj.method().unwrap(), 42);
    let d = [("obj", obj)].into_py_dict(py);
    py.run("assert obj.method() == 42", None, Some(d)).unwrap();
    py.run("assert obj.method.__doc__ == 'Test method'", None, Some(d))
        .unwrap();
}

#[pyclass]
struct InstanceMethodWithArgs {
    member: i32,
}

#[pymethods]
impl InstanceMethodWithArgs {
    fn method(&self, multiplier: i32) -> PyResult<i32> {
        Ok(self.member * multiplier)
    }
}

#[test]
#[allow(dead_code)]
fn instance_method_with_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let obj = PyClassShell::new_mut(py, InstanceMethodWithArgs { member: 7 }).unwrap();
    assert_eq!(obj.method(6).unwrap(), 42);
    let d = [("obj", obj)].into_py_dict(py);
    py.run("assert obj.method(3) == 21", None, Some(d)).unwrap();
    py.run("assert obj.method(multiplier=6) == 42", None, Some(d))
        .unwrap();
}

#[pyclass]
struct ClassMethod {}

#[pymethods]
impl ClassMethod {
    #[new]
    fn new() -> Self {
        ClassMethod {}
    }

    #[classmethod]
    /// Test class method.
    fn method(cls: &PyType) -> PyResult<String> {
        Ok(format!("{}.method()!", cls.name()))
    }
}

#[test]
fn class_method() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let d = [("C", py.get_type::<ClassMethod>())].into_py_dict(py);
    py.run(
        "assert C.method() == 'ClassMethod.method()!'",
        None,
        Some(d),
    )
    .unwrap();
    py.run(
        "assert C().method() == 'ClassMethod.method()!'",
        None,
        Some(d),
    )
    .unwrap();
    py.run(
        "assert C.method.__doc__ == 'Test class method.'",
        None,
        Some(d),
    )
    .unwrap();
    py.run(
        "assert C().method.__doc__ == 'Test class method.'",
        None,
        Some(d),
    )
    .unwrap();
}

#[pyclass]
struct ClassMethodWithArgs {}

#[pymethods]
impl ClassMethodWithArgs {
    #[classmethod]
    fn method(cls: &PyType, input: &PyString) -> PyResult<String> {
        Ok(format!("{}.method({})", cls.name(), input))
    }
}

#[test]
fn class_method_with_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let d = [("C", py.get_type::<ClassMethodWithArgs>())].into_py_dict(py);
    py.run(
        "assert C.method('abc') == 'ClassMethodWithArgs.method(abc)'",
        None,
        Some(d),
    )
    .unwrap();
}

#[pyclass]
struct StaticMethod {}

#[pymethods]
impl StaticMethod {
    #[new]
    fn new() -> Self {
        StaticMethod {}
    }

    #[staticmethod]
    /// Test static method.
    fn method(_py: Python) -> PyResult<&'static str> {
        Ok("StaticMethod.method()!")
    }
}

#[test]
fn static_method() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    assert_eq!(StaticMethod::method(py).unwrap(), "StaticMethod.method()!");

    let d = [("C", py.get_type::<StaticMethod>())].into_py_dict(py);
    py.run(
        "assert C.method() == 'StaticMethod.method()!'",
        None,
        Some(d),
    )
    .unwrap();
    py.run(
        "assert C().method() == 'StaticMethod.method()!'",
        None,
        Some(d),
    )
    .unwrap();
    py.run(
        "assert C.method.__doc__ == 'Test static method.'",
        None,
        Some(d),
    )
    .unwrap();
    py.run(
        "assert C().method.__doc__ == 'Test static method.'",
        None,
        Some(d),
    )
    .unwrap();
}

#[pyclass]
struct StaticMethodWithArgs {}

#[pymethods]
impl StaticMethodWithArgs {
    #[staticmethod]
    fn method(_py: Python, input: i32) -> PyResult<String> {
        Ok(format!("0x{:x}", input))
    }
}

#[test]
fn static_method_with_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    assert_eq!(StaticMethodWithArgs::method(py, 1234).unwrap(), "0x4d2");

    let d = [("C", py.get_type::<StaticMethodWithArgs>())].into_py_dict(py);
    py.run("assert C.method(1337) == '0x539'", None, Some(d))
        .unwrap();
}

#[pyclass]
struct MethArgs {}

#[pymethods]
impl MethArgs {
    #[args(test)]
    fn get_optional(&self, test: Option<i32>) -> PyResult<i32> {
        Ok(test.unwrap_or(10))
    }
    fn get_optional2(&self, test: Option<i32>) -> PyResult<Option<i32>> {
        Ok(test)
    }
    #[args(test = "None")]
    fn get_optional3(&self, test: Option<i32>) -> PyResult<Option<i32>> {
        Ok(test)
    }
    fn get_optional_positional(
        &self,
        _t1: Option<i32>,
        t2: Option<i32>,
        _t3: Option<i32>,
    ) -> PyResult<Option<i32>> {
        Ok(t2)
    }

    #[args(test = "10")]
    fn get_default(&self, test: i32) -> PyResult<i32> {
        Ok(test)
    }
    #[args("*", test = 10)]
    fn get_kwarg(&self, test: i32) -> PyResult<i32> {
        Ok(test)
    }
    #[args(args = "*", kwargs = "**")]
    fn get_kwargs(
        &self,
        py: Python,
        args: &PyTuple,
        kwargs: Option<&PyDict>,
    ) -> PyResult<PyObject> {
        Ok([args.into(), kwargs.to_object(py)].to_object(py))
    }

    #[args(args = "*", kwargs = "**")]
    fn get_pos_arg_kw(
        &self,
        py: Python,
        a: i32,
        args: &PyTuple,
        kwargs: Option<&PyDict>,
    ) -> PyObject {
        [a.to_object(py), args.into(), kwargs.to_object(py)].to_object(py)
    }

    #[args(kwargs = "**")]
    fn get_pos_kw(&self, py: Python, a: i32, kwargs: Option<&PyDict>) -> PyObject {
        [a.to_object(py), kwargs.to_object(py)].to_object(py)
    }

    // "args" can be anything that can be extracted from PyTuple
    #[args(args = "*")]
    fn args_as_vec(&self, args: Vec<i32>) -> i32 {
        args.iter().sum()
    }
}

#[test]
fn meth_args() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let inst = Py::new(py, MethArgs {}).unwrap();

    py_run!(py, inst, "assert inst.get_optional() == 10");
    py_run!(py, inst, "assert inst.get_optional(100) == 100");
    py_run!(py, inst, "assert inst.get_optional2() == None");
    py_run!(py, inst, "assert inst.get_optional2(100) == 100");
    py_run!(py, inst, "assert inst.get_optional3() == None");
    py_run!(py, inst, "assert inst.get_optional3(100) == 100");
    py_run!(
        py,
        inst,
        "assert inst.get_optional_positional(1, 2, 3) == 2"
    );
    py_run!(py, inst, "assert inst.get_optional_positional(1) == None");
    py_run!(py, inst, "assert inst.get_default() == 10");
    py_run!(py, inst, "assert inst.get_default(100) == 100");
    py_run!(py, inst, "assert inst.get_kwarg() == 10");
    py_run!(py, inst, "assert inst.get_kwarg(100) == 10");
    py_run!(py, inst, "assert inst.get_kwarg(test=100) == 100");
    py_run!(py, inst, "assert inst.get_kwargs() == [(), None]");
    py_run!(py, inst, "assert inst.get_kwargs(1,2,3) == [(1,2,3), None]");
    py_run!(
        py,
        inst,
        "assert inst.get_kwargs(t=1,n=2) == [(), {'t': 1, 'n': 2}]"
    );
    py_run!(
        py,
        inst,
        "assert inst.get_kwargs(1,2,3,t=1,n=2) == [(1,2,3), {'t': 1, 'n': 2}]"
    );

    py_run!(py, inst, "assert inst.get_pos_arg_kw(1) == [1, (), None]");
    py_run!(
        py,
        inst,
        "assert inst.get_pos_arg_kw(1, 2, 3) == [1, (2, 3), None]"
    );
    py_run!(
        py,
        inst,
        "assert inst.get_pos_arg_kw(1, b=2) == [1, (), {'b': 2}]"
    );
    py_run!(py, inst, "assert inst.get_pos_arg_kw(a=1) == [1, (), None]");
    py_expect_exception!(py, inst, "inst.get_pos_arg_kw()", TypeError);
    py_expect_exception!(py, inst, "inst.get_pos_arg_kw(1, a=1)", TypeError);
    py_expect_exception!(py, inst, "inst.get_pos_arg_kw(b=2)", TypeError);

    py_run!(py, inst, "assert inst.get_pos_kw(1, b=2) == [1, {'b': 2}]");
    py_expect_exception!(py, inst, "inst.get_pos_kw(1,2)", TypeError);

    py_run!(py, inst, "assert inst.args_as_vec(1,2,3) == 6");
}

#[pyclass]
/// A class with "documentation".
struct MethDocs {
    x: i32,
}

#[pymethods]
impl MethDocs {
    /// A method with "documentation" as well.
    fn method(&self) -> PyResult<i32> {
        Ok(0)
    }

    #[getter]
    /// `int`: a very "important" member of 'this' instance.
    fn get_x(&self) -> PyResult<i32> {
        Ok(self.x)
    }
}

#[test]
fn meth_doc() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let d = [("C", py.get_type::<MethDocs>())].into_py_dict(py);

    py.run(
        "assert C.__doc__ == 'A class with \"documentation\".'",
        None,
        Some(d),
    )
    .unwrap();
    py.run(
        "assert C.method.__doc__ == 'A method with \"documentation\" as well.'",
        None,
        Some(d),
    )
    .unwrap();
    py.run(
        "assert C.x.__doc__ == '`int`: a very \"important\" member of \\'this\\' instance.'",
        None,
        Some(d),
    )
    .unwrap();
}

#[pyclass]
struct MethodWithLifeTime {}

#[pymethods]
impl MethodWithLifeTime {
    fn set_to_list<'py>(&self, py: Python<'py>, set: &'py PySet) -> PyResult<&'py PyList> {
        let mut items = vec![];
        for _ in 0..set.len() {
            items.push(set.pop().unwrap());
        }
        let list = PyList::new(py, items);
        list.sort()?;
        Ok(list)
    }
}

#[test]
fn method_with_lifetime() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let obj = PyClassShell::new_ref(py, MethodWithLifeTime {}).unwrap();
    py_run!(
        py,
        obj,
        "assert obj.set_to_list(set((1, 2, 3))) == [1, 2, 3]"
    );
}
