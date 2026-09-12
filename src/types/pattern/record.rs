use crate::types::Pattern;

/// A named record field pattern, like `name: "Dave"` or `age: n Int`.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RecordPattern {
    pub name: String,
    pub value: Box<Pattern>,
}

impl RecordPattern {
    pub fn desugar(self) -> RecordPattern {
        self
    }
}

impl std::fmt::Display for RecordPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}
