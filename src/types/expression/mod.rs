use crate::types::{Token, Literal, Pattern};
use crate::type_system::Typed;
use crate::parser::{ParserError};

use std::collections::HashMap;

mod conditional;
mod infix;
mod method;
mod prefix;

pub use self::conditional::Conditional;
pub use self::infix::Infix;
pub use self::method::{Method, Call};
pub use self::prefix::Prefix;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub lexeme: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionKind {
    Conditional(Conditional),
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    /// A value, tuple, record, field or variable pattern.
    Pattern(Box<dyn Pattern>),
    /// A reference to a type like `Int32`.
    Type,
    /// An expression with a prefix operator.
    Prefix(Prefix),
    Infix(Infix),
    Call(Call),
    /// A definition of a multimethod.
    Method(Method),
    /// A first-class chunk of code that can be passed around as a value.
    Block(Vec<Expression>),
    /// One or more pattern enclosed in parentheses.
    Group(Box<Expression>),
    Identifier,
}