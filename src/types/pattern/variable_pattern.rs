//use crate::types::{Expression};

/// A variable identifier with an optional name and type.
///
/// The pattern consists of an identifier and an optional following type, and if the 
/// name happens to be `_`, an underscore character, then the variable has no name.
pub struct VariablePattern {
    pub name:    Option<String>,
    pub type_id: Option<String>,
}