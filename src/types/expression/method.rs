use crate::types::*;

/// An expression which defines a multimethod.
///
/// A method can be registered to the same name multiple times if the signature is not already
/// present. When it is called in the interpreter, we check if the call signature matches with one
/// of the defined method's signatures, and if it does, execute that function's body.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Method {
    /// The name by which this multimethod is referenced.
    pub name: String,
    /// The method signature which defines the arguments.
    pub signature: Option<Pattern>,
    pub body: Box<Expression>,
}

/// A call of a method with a given set of arguments.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Call {
    /// The name of the multimethod being called.
    pub name: String,
    // The [`Record`] which contains the values of the arguments of the method call.
    pub signature:  Option<Pattern>,
}