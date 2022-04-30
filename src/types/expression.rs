use crate::types::{Token, Literal};

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
    Identifier,
}

/// An expression with a prefix operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prefix {
    pub operator: Token,
    pub operand:  Box<Expression>,
}

/// An expression which defines a multimethod.
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    /// An expression that evaluates to a value.
    Value {
        expr: Box<Expression>
    },
    /// An unnamed series of patterns separated by commas.
    Tuple {
        expr: Box<Expression>,
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