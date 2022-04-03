use crate::token::{Token, Literal};

use std::collections::BTreeMap;

type VariablePatternName = Option<String>;
type VariablePatternType = Option<String>;

pub enum Expression<'a> {
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    /// A value, tuple, record or variable pattern.
    Pattern(Pattern<'a>),
    /// A reference to a type like `Int32`.
    Type(String),
    /// An expression with a prefix operator.
    Unary(UnaryExpression<'a>),
    Identifier(String),
}

/// An expression with a prefix operator.
pub struct UnaryExpression<'a> {
    pub operator: Token,
    pub expr:     &'a Expression<'a>,
}

/// An expression with two child expressions and an operator in between.
pub struct BinaryExpression<'a> {
    pub operator: Token,
    pub left:     &'a Expression<'a>,
    pub right:    &'a Expression<'a>,
}

pub enum Pattern<'a> {
    /// An expression that evaluates to a value.
    Value(&'a Expression<'a>),
    /// A series of patterns separated by commas.
    Tuple(Vec<Pattern<'a>>),
    /// A named series of patterns separated by commas.
    Record(BTreeMap<String, Pattern<'a>>),
    /// A variable identifier with optional name and type.
    Variable(VariablePatternName, VariablePatternType),
}