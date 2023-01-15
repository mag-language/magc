use crate::types::{Expression, Pattern};
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
    /// Contains all method instances defined at runtime.
    ///
    /// The `Multimethod` type in this environment stores an arbitrary number of pairs
    /// of method signatures and bodies under a single name, provides methods to match
    /// its signatures with a given call signature and extracts any variables.
    multimethods: Environment<Multimethod>,
    /// Contains all infix operators defined at runtime.
    infix_ops:    Environment<InfixOperatorDefinition>,
    /// Contains all prefix operators defined at runtime.
    prefix_ops:   Environment<PrefixOperatorDefinition>,
    /// A structure which keeps track of defined types.
    types:        TypeSystem,
    /// Reports errors to the user with helpful information.
    errors:       ErrorReporter,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            variables:    HashMap::new(),
            multimethods: HashMap::new(),
            infix_ops:    HashMap::new(),
            prefix_ops:   HashMap::new(),
            types:        TypeSystem,
            errors:       ErrorReporter,
        }
    }
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