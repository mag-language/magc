use crate::types::{Expression, Pattern};
use std::collections::HashMap;

pub type Environment<T> = HashMap<String, T>;

mod compilelets;
mod errors;
mod type_system;
mod multimethod;

pub use self::compilelets::Compilelet;
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
    /// Maps expression types to pieces of code able to compile that specific expression.
    compilelets: HashMap<String, &'static dyn Compilelet>,
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
            compilelets:  HashMap::new(),
            types:        TypeSystem,
            errors:       ErrorReporter,
        }
    }
}
