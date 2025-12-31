use super::error::{ErrorKind, RopeError};
use super::operator::Operator;
use super::result::{err, ok, ResultObj};
use crate::data::{py_to_value, value_to_py};
use crate::ops::PathItem;
use crate::ops::{apply, OpError, OperatorKind};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyType};
use std::collections::HashMap;

#[pyclass]
#[derive(Clone)]
pub struct Blueprint {
    pub ops: Vec<OperatorKind>,
}

#[pymethods]
impl Blueprint {
    #[new]
    fn new() -> Self {
        Blueprint { ops: Vec::new() }
    }

    #[classmethod]
    fn for_type(_cls: &Bound<'_, PyType>, _ty: &Bound<'_, PyAny>) -> Self {
        Blueprint { ops: Vec::new() }
    }

    #[classmethod]
    fn any(_cls: &Bound<'_, PyType>) -> Self {
        Blueprint { ops: Vec::new() }
    }

    fn pipe(&self, op: PyRef<'_, Operator>) -> Self {
        let mut ops = self.ops.clone();
        ops.push(op.kind.clone());
        Blueprint { ops }
    }

    /// Convenience method equivalent to `.pipe(Op.coerce.assert_str())`.
    /// Narrows the output type to `str` by asserting the value is a string.
    ///
    /// Note: If more guard methods are needed (e.g., guard_int, guard_list),
    /// consider generating them via gen_ops.py based on @ns coerce operators.
    fn guard_str(&self) -> Self {
        let mut ops = self.ops.clone();
        ops.push(OperatorKind::AssertStr);
        Blueprint { ops }
    }

    fn __repr__(&self) -> String {
        format!("Blueprint(ops={})", self.ops.len())
    }
}

#[pyfunction]
pub fn run(py: Python<'_>, blueprint: PyRef<'_, Blueprint>, input: Py<PyAny>) -> ResultObj {
    let mut current = match py_to_value(input.bind(py)) {
        Ok(value) => value,
        Err(e) => {
            return rope_error(
                py,
                ErrorKind::InvalidInput,
                e.code,
                e.message,
                None,
                Some("Input".to_string()),
                Vec::new(),
                Some(e.expected.to_string()),
                Some(e.got),
                None,
            )
        }
    };
    for op in &blueprint.ops {
        match apply(op, current) {
            Ok(value) => current = value,
            Err(e) => return op_error_to_result(py, e),
        }
    }
    ok(value_to_py(py, current))
}

fn op_error_to_result(py: Python<'_>, e: OpError) -> ResultObj {
    rope_error(
        py,
        e.kind, // No conversion needed - same ErrorKind type
        e.code,
        e.message,
        None,
        Some(e.op.to_string()),
        e.path,
        e.expected.map(|s| s.to_string()),
        e.got,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn rope_error(
    py: Python<'_>,
    kind: ErrorKind,
    code: &str,
    message: &str,
    metadata: Option<HashMap<String, String>>,
    op: Option<String>,
    path: Vec<PathItem>,
    expected: Option<String>,
    got: Option<String>,
    cause: Option<String>,
) -> ResultObj {
    let err_obj = Py::new(
        py,
        RopeError {
            kind,
            code: code.to_string(),
            message: message.to_string(),
            metadata: metadata.unwrap_or_default(),
            op,
            path,
            expected,
            got,
            cause,
        },
    )
    .expect("rope error alloc");
    err(err_obj.into())
}
