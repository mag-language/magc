use crate::token::{Token, Literal};

use std::collections::BTreeMap;

type VariablePatternName = Option<String>;
type VariablePatternType = Option<String>;

#[derive(Debug, Clone)]
pub struct Expression<'a> {
    pub kind: ExpressionKind<'a>,
    pub lexeme: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug, Clone)]
pub enum ExpressionKind<'a> {
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    /// A value, tuple, record or variable pattern.
    Pattern(Pattern<'a>),
    /// A reference to a type like `Int32`.
    Type,
    /// An expression with a prefix operator.
    Unary(UnaryExpression<'a>),
    Identifier,
}

/// An expression with a prefix operator.
#[derive(Debug, Clone)]
pub struct UnaryExpression<'a> {
    pub operator: Token,
    pub expr:     &'a Expression<'a>,
}

/// An expression with two child expressions and an operator in between.
#[derive(Debug, Clone)]
pub struct BinaryExpression<'a> {
    pub operator: Token,
    pub left:     &'a Expression<'a>,
    pub right:    &'a Expression<'a>,
}

#[derive(Debug, Clone)]
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