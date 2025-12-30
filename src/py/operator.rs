use crate::ops::OperatorKind;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Clone)]
pub struct Operator {
    pub kind: OperatorKind,
}

#[pymethods]
impl Operator {
    fn __repr__(&self) -> String {
        format!("Operator.{}", self.kind.name())
    }
}

// Operator factory functions for Python
#[pyfunction(name = "_op_assert_str")]
pub fn op_assert_str() -> Operator {
    Operator {
        kind: OperatorKind::AssertStr,
    }
}

#[pyfunction(name = "_op_split")]
pub fn op_split(delim: String) -> Operator {
    Operator {
        kind: OperatorKind::Split { delim },
    }
}

#[pyfunction(name = "_op_index")]
pub fn op_index(idx: usize) -> Operator {
    Operator {
        kind: OperatorKind::Index { idx },
    }
}

#[pyfunction(name = "_op_get_key")]
pub fn op_get_key(key: String) -> Operator {
    Operator {
        kind: OperatorKind::GetKey { key },
    }
}

#[pyfunction(name = "_op_to_uppercase")]
pub fn op_to_uppercase() -> Operator {
    Operator {
        kind: OperatorKind::ToUppercase,
    }
}
