use crate::types::{Expression, VariablePattern};
use std::collections::HashMap;

pub type Environment<T> = HashMap<String, T>;

pub struct Compiler {
    /// The global namespace for variables.
    variables:    Environment<Expression>,
    multimethods: Environment<Multimethod>,
}

pub struct Multimethod;