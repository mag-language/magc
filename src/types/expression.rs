use crate::types::{Token, Literal, Pattern};
use crate::type_system::Typed;
use crate::parser::{ParserError};

use std::collections::HashMap;

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
    Pattern(Pattern),
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

/// An expression with a prefix operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prefix {
    pub operator: Token,
    pub operand:  Box<Expression>,
}

/// An expression which defines a multimethod.
///
/// This method can be registered to the same name multiple times if the signature is not already
/// present. When it is called in the interpreter, we check if the call signature matches with one
/// of the defined method's signatures, and if it does, execute that function's body.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Method {
    /// The name by which this multimethod is referenced.
    pub name: String,
    /// The method signature which defines the arguments.
    pub signature: Box<Expression>,
    pub body: Box<Expression>,
}

/// An expression with a prefix operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Call {
    pub method: Box<Expression>,
    // The [`Record`] which contains the values of the arguments of the method call.
    pub signature:  Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub then_arm:  Box<Expression>,
    pub else_arm:  Option<Box<Expression>>,
}

/// An expression with a infix operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Infix {
    pub left:  Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

/// An expression with two child expressions and an operator in between.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpression {
    pub operator: Token,
    pub left:     Box<Expression>,
    pub right:    Box<Expression>,
}