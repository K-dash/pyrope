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
