use crate::types::{Expression, VariablePattern, Pattern};
use std::collections::HashMap;

pub type Environment<T> = HashMap<String, T>;

pub struct Compiler {
    /// The global namespace for variables.
    variables:    Environment<Expression>,
    multimethods: Environment<Multimethod>,
    infix_ops:    Environment<InfixOperatorDefinition>,
    prefix_ops:   Environment<PrefixOperatorDefinition>,
    types:        TypeSystem,
    errors:       ErrorReporter,
}

pub struct TypeSystem;
pub struct ErrorReporter;
pub struct Multimethod;

pub struct InfixOperatorDefinition {
    pub precedence: usize,
    pub signature: Option<Pattern>,
    pub body: Vec<Expression>,
}

pub struct PrefixOperatorDefinition {
    pub signature: Option<Pattern>,
    pub body: Vec<Expression>,
}