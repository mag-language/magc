/// A variable identifier with an optional type annotation.
///
/// If the name of the identifier happens to be a single underscore character,
/// the variable is considered nameless and no destructuring will take place.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VariablePattern {
    pub name:    Option<String>,
    pub type_id: Option<String>,
}

impl VariablePattern {
    pub fn desugar(mut self) -> VariablePattern {
        self
    }
}