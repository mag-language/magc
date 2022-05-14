use crate::types::{Literal, Pattern};

mod conditional;
mod infix;
mod method;
mod prefix;

pub use self::conditional::Conditional;
pub use self::infix::Infix;
pub use self::method::{Method, Call};
pub use self::prefix::Prefix;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub lexeme: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExpressionKind {
    /// An `if` expression running different branches of code based on a given condition.
    Conditional(Conditional),
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    /// A value, tuple, field or variable pattern.
    Pattern(Pattern),
    /// A reference to a type like `Int32`.
    Type,
    /// An expression with a prefix operator.
    Prefix(Prefix),
    /// Two expressions with an infix operator in between.
    Infix(Infix),
    /// An invocation of a method, like `print("Hello, World!")`
    Call(Call),
    /// A definition of a multimethod.
    Method(Method),
    /// A first-class chunk of code that can be passed around as a value.
    Block(Vec<Expression>),
    Identifier,
}