use crate::types::{Expression, VariablePattern, Pattern};
use std::collections::HashMap;

pub type Environment<T> = HashMap<String, T>;

mod errors;
mod type_system;
mod multimethod;

pub use self::errors::ErrorReporter;
pub use self::multimethod::Multimethod;
pub use self::type_system::TypeSystem;

pub struct Compiler {
    /// The global namespace for variables.
    variables:    Environment<Expression>,
    multimethods: Environment<Multimethod>,
    infix_ops:    Environment<InfixOperatorDefinition>,
    prefix_ops:   Environment<PrefixOperatorDefinition>,
    types:        TypeSystem,
    errors:       ErrorReporter,
}

pub struct InfixOperatorDefinition {
    pub precedence: usize,
    pub signature: Option<Pattern>,
    pub body: Vec<Expression>,
}

pub struct PrefixOperatorDefinition {
    pub signature: Option<Pattern>,
    pub body: Vec<Expression>,
}