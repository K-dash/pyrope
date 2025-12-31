#[derive(Clone, Debug)]
pub enum OperatorKind {
    /// @op name=assert_str py=assert_str
    /// @sig in=object out=str
    AssertStr,

    /// @op name=expect_str py=expect_str
    /// @sig in=object out=str
    ExpectStr,

    /// @op name=split py=split
    /// @sig in=str out=list[str]
    /// @param delim:str
    Split { delim: String },

    /// @op name=index py=index
    /// @sig in=Sequence[object] out=object
    /// @param idx:int
    Index { idx: usize },

    /// @op name=get py=get
    /// @sig in=Mapping[str, object] out=object
    /// @param key:str
    GetKey { key: String },

    /// @op name=to_uppercase py=to_uppercase
    /// @sig in=str out=str
    ToUppercase,

    /// @op name=len py=len
    /// @sig in=str out=int
    Len,
}

impl OperatorKind {
    pub fn name(&self) -> &'static str {
        match self {
            OperatorKind::AssertStr => "AssertStr",
            OperatorKind::Split { .. } => "Split",
            OperatorKind::Index { .. } => "Index",
            OperatorKind::GetKey { .. } => "GetKey",
            OperatorKind::ToUppercase => "ToUppercase",
            OperatorKind::ExpectStr => "ExpectStr",
            OperatorKind::Len => "Len",
        }
    }
}
