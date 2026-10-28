/// An open document instance.
#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub struct Document {
    pub id: String,
    pub name: String,
    pub kind: &'static str,
}

impl Document {
    pub fn new(id: impl Into<String>, name: impl Into<String>, kind: &'static str) -> Self {
        Self { id: id.into(), name: name.into(), kind }
    }
}
