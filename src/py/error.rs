use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList, PyString};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum PathItem {
    Key(String),
    Index(usize),
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    InvalidInput,
    NotFound,
    Internal,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::InvalidInput => "InvalidInput",
            ErrorKind::NotFound => "NotFound",
            ErrorKind::Internal => "Internal",
        }
    }
}

#[pyclass(frozen, name = "ErrorKind")]
#[derive(Clone)]
pub struct ErrorKindObj {
    pub kind: ErrorKind,
}

#[pymethods]
#[allow(non_snake_case)]
impl ErrorKindObj {
    #[classattr]
    fn InvalidInput(py: Python<'_>) -> Py<ErrorKindObj> {
        Py::new(
            py,
            ErrorKindObj {
                kind: ErrorKind::InvalidInput,
            },
        )
        .expect("ErrorKind alloc")
    }

    #[classattr]
    fn NotFound(py: Python<'_>) -> Py<ErrorKindObj> {
        Py::new(
            py,
            ErrorKindObj {
                kind: ErrorKind::NotFound,
            },
        )
        .expect("ErrorKind alloc")
    }

    #[classattr]
    fn Internal(py: Python<'_>) -> Py<ErrorKindObj> {
        Py::new(
            py,
            ErrorKindObj {
                kind: ErrorKind::Internal,
            },
        )
        .expect("ErrorKind alloc")
    }

    fn __repr__(&self) -> String {
        format!("ErrorKind.{}", self.kind.as_str())
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __eq__(&self, other: PyRef<'_, ErrorKindObj>) -> bool {
        self.kind.as_str() == other.kind.as_str()
    }
}

#[pyclass(frozen)]
#[derive(Clone)]
pub struct RopeError {
    pub kind: ErrorKind,
    pub code: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
    pub op: Option<String>,
    pub path: Vec<PathItem>,
    pub expected: Option<String>,
    pub got: Option<String>,
    pub cause: Option<String>,
}

#[pymethods]
impl RopeError {
    #[getter]
    fn kind(&self, py: Python<'_>) -> Py<ErrorKindObj> {
        Py::new(py, ErrorKindObj { kind: self.kind }).expect("ErrorKind alloc")
    }

    #[getter]
    fn code(&self) -> String {
        self.code.clone()
    }

    #[getter]
    fn message(&self) -> String {
        self.message.clone()
    }

    #[getter]
    fn metadata(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let dict = PyDict::new(py);
        for (k, v) in &self.metadata {
            dict.set_item(k, v)?;
        }
        Ok(dict.into())
    }

    #[getter]
    fn op(&self) -> Option<String> {
        self.op.clone()
    }

    #[getter]
    fn path(&self, py: Python<'_>) -> Py<PyAny> {
        let list = PyList::empty(py);
        for item in &self.path {
            match item {
                PathItem::Key(value) => {
                    list.append(PyString::new(py, value)).expect("path key");
                }
                PathItem::Index(value) => {
                    list.append(*value).expect("path index");
                }
            }
        }
        list.unbind().into()
    }

    #[getter]
    fn expected(&self) -> Option<String> {
        self.expected.clone()
    }

    #[getter]
    fn got(&self) -> Option<String> {
        self.got.clone()
    }

    #[getter]
    fn cause(&self) -> Option<String> {
        self.cause.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "RopeError(kind=ErrorKind.{}, code='{}', message='{}')",
            self.kind.as_str(),
            self.code,
            self.message
        )
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}
