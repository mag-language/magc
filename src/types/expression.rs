use crate::types::{Token, Literal};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub lexeme: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    Conditional(Conditional),
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    /// A value, tuple, record or variable pattern.
    Pattern(Pattern),
    /// A reference to a type like `Int32`.
    Type,
    /// An expression with a prefix operator.
    Prefix(Prefix),
    Infix(Infix),
    Call(Call),
    Method(Method),
    Identifier,
}

/// An expression with a prefix operator.
#[derive(Debug, Clone, PartialEq)]
pub struct Prefix {
    pub operator: Token,
    pub operand:  Box<Expression>,
}

/// An expression which defines a multimethod.
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    /// The name by which this multimethod is referenced.
    pub name: String,
    /// The method signature which defines the arguments.
    pub signature: Box<Expression>,
    pub body: Vec<Expression>,
}

/// An expression with a prefix operator.
#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub method: Box<Expression>,
    // The [`Record`] which contains the values of the arguments of the method call.
    pub signature:  Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub then_arm:  Box<Expression>,
    pub else_arm:  Option<Box<Expression>>,
}

/// An expression with a infix operator.
#[derive(Debug, Clone, PartialEq)]
pub struct Infix {
    pub left:  Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

/// An expression with two child expressions and an operator in between.
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub operator: Token,
    pub left:     Box<Expression>,
    pub right:    Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// An expression that evaluates to a value.
    Value {
        expr: Box<Expression>
    },
    /// An unnamed series of patterns separated by commas.
    Tuple {
        children: Vec<Expression>,
    },
    /// A named series of patterns separated by commas.
    Record {
        children: Vec<Expression>,
    },

    /// The smallest possible unit of a record, like `repeats: 4`.
    Field {
        name: String,
        value: Box<Expression>,
    },
    /// A variable identifier with optional name and type.
    Variable {
        name: Option<String>,
        type_id: Option<String>,
    },
}