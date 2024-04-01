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
    pub fn desugar(self) -> VariablePattern {
        self
    }
}

impl std::fmt::Display for VariablePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", name)?;
        } else {
            write!(f, "_")?;
        }

        if let Some(type_id) = &self.type_id {
            write!(f, ": {}", type_id)?;
        }

        Ok(())
    }
}