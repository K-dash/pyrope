use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::Bound;

#[pyclass(name = "Result")]
pub struct ResultObj {
    pub is_ok: bool,
    pub ok: Option<Py<PyAny>>,
    pub err: Option<Py<PyAny>>,
}

#[pymethods]
impl ResultObj {
    fn is_ok(&self) -> bool {
        self.is_ok
    }

    fn is_err(&self) -> bool {
        !self.is_ok
    }

    fn unwrap(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        if self.is_ok {
            Ok(self.ok.as_ref().expect("ok value").clone_ref(py))
        } else {
            Err(PyRuntimeError::new_err("called unwrap() on Err"))
        }
    }

    fn unwrap_err(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        if self.is_ok {
            Err(PyRuntimeError::new_err("called unwrap_err() on Ok"))
        } else {
            Ok(self.err.as_ref().expect("err value").clone_ref(py))
        }
    }

    fn map(&self, py: Python<'_>, f: Bound<'_, PyAny>) -> PyResult<Self> {
        if self.is_ok {
            let value = self.ok.as_ref().expect("ok value");
            let mapped = f.call1((value.clone_ref(py),))?;
            Ok(ok(mapped.into()))
        } else {
            Ok(err(self.err.as_ref().expect("err value").clone_ref(py)))
        }
    }

    fn map_err(&self, py: Python<'_>, f: Bound<'_, PyAny>) -> PyResult<Self> {
        if self.is_ok {
            Ok(ok(self.ok.as_ref().expect("ok value").clone_ref(py)))
        } else {
            let value = self.err.as_ref().expect("err value");
            let mapped = f.call1((value.clone_ref(py),))?;
            Ok(err(mapped.into()))
        }
    }

    fn and_then(&self, py: Python<'_>, f: Bound<'_, PyAny>) -> PyResult<Self> {
        if self.is_ok {
            let value = self.ok.as_ref().expect("ok value");
            let out = f.call1((value.clone_ref(py),))?;
            let result_type = py.get_type::<ResultObj>();
            if !out.is_instance(result_type.as_any())? {
                return Err(PyTypeError::new_err("and_then callback must return Result"));
            }
            let out_ref: PyRef<'_, ResultObj> = out.extract()?;
            Ok(ResultObj {
                is_ok: out_ref.is_ok,
                ok: out_ref.ok.as_ref().map(|v| v.clone_ref(py)),
                err: out_ref.err.as_ref().map(|v| v.clone_ref(py)),
            })
        } else {
            Ok(err(self.err.as_ref().expect("err value").clone_ref(py)))
        }
    }
}

#[pyclass(name = "Option")]
pub struct OptionObj {
    pub is_some: bool,
    pub value: Option<Py<PyAny>>,
}

#[pymethods]
impl OptionObj {
    fn is_some(&self) -> bool {
        self.is_some
    }

    fn is_none(&self) -> bool {
        !self.is_some
    }

    fn unwrap(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        if self.is_some {
            Ok(self.value.as_ref().expect("some value").clone_ref(py))
        } else {
            Err(PyRuntimeError::new_err("called unwrap() on None_"))
        }
    }

    fn map(&self, py: Python<'_>, f: Bound<'_, PyAny>) -> PyResult<Self> {
        if self.is_some {
            let value = self.value.as_ref().expect("some value");
            let mapped = f.call1((value.clone_ref(py),))?;
            Ok(some(mapped.into()))
        } else {
            Ok(none_())
        }
    }

    fn unwrap_or(&self, py: Python<'_>, default: Py<PyAny>) -> PyResult<Py<PyAny>> {
        if self.is_some {
            Ok(self.value.as_ref().expect("some value").clone_ref(py))
        } else {
            Ok(default.clone_ref(py))
        }
    }
}

// Python-facing constructor functions
#[pyfunction(name = "Ok")]
pub fn py_ok(value: Py<PyAny>) -> ResultObj {
    ok(value)
}

#[pyfunction(name = "Err")]
pub fn py_err(error: Py<PyAny>) -> ResultObj {
    err(error)
}

#[pyfunction(name = "Some")]
pub fn py_some(value: Py<PyAny>) -> OptionObj {
    some(value)
}

#[pyfunction(name = "None_")]
pub fn py_none() -> OptionObj {
    none_()
}

// Internal constructor functions
pub fn ok(value: Py<PyAny>) -> ResultObj {
    ResultObj {
        is_ok: true,
        ok: Some(value),
        err: None,
    }
}

pub fn err(error: Py<PyAny>) -> ResultObj {
    ResultObj {
        is_ok: false,
        ok: None,
        err: Some(error),
    }
}

pub fn some(value: Py<PyAny>) -> OptionObj {
    OptionObj {
        is_some: true,
        value: Some(value),
    }
}

pub fn none_() -> OptionObj {
    OptionObj {
        is_some: false,
        value: None,
    }
}
