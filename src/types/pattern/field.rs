use crate::types::Pattern;

/// A named pattern, like `repeats: 4` or `name: n String`.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FieldPattern {
    pub name:  String,
    pub value: Box<Pattern>,
}

impl FieldPattern {
    pub fn desugar(self) -> FieldPattern {
        self
    }
}

impl std::fmt::Display for FieldPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}