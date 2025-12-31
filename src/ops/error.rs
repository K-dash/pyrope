/// Error kind enum shared between ops and py layers.
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub enum PathItem {
    Key(String),
    Index(usize),
}

#[derive(Debug)]
pub struct OpError {
    pub kind: ErrorKind,
    pub code: &'static str,
    pub message: &'static str,
    pub op: &'static str,
    pub path: Vec<PathItem>,
    pub expected: Option<&'static str>,
    pub got: Option<String>,
}

impl OpError {
    pub fn type_mismatch(op: &'static str, expected: &'static str, got: String) -> Self {
        OpError {
            kind: ErrorKind::InvalidInput,
            code: "type_mismatch",
            message: "Type mismatch",
            op,
            path: Vec::new(),
            expected: Some(expected),
            got: Some(got),
        }
    }
}
