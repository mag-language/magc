use crate::types::{
    Environment,
    Expression,
    Pattern,
};

/// A variable identifier with an optional type annotation.
///
/// If the name of the identifier happens to be a single underscore character,
/// the variable is considered nameless and no destructuring will take place.
pub struct VariablePattern {
    pub name:    Option<String>,
    pub type_id: Option<String>,
}

impl Pattern for VariablePattern {
    fn match_with(&self, expression: Box<Expression>) -> Option<Environment> {
        // We don't really have a type system yet, so let's ignore the pattern's
        // type identifier for now and just extract any expression into a new
        // environment without doing type checks.
        if let Some(name) = self.name {
            // Add an entry with this pattern's name and the given expression into
            // the environment so we can use the variable later.
            Some(
                HashMap::new().insert(name, expression)
            )
        } else {
            // Return an empty environment since the name is `_`.
            Some(
                HashMap::new()
            )
        }
    }
}