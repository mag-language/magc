use crate::token::{Token, Literal};

use std::collections::BTreeMap;

type VariablePatternName = Option<String>;
type VariablePatternType = Option<String>;

#[derive(Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub lexeme: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Conditional(ConditionalExpression),
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    /// A value, tuple, record or variable pattern.
    Pattern(Pattern),
    /// A reference to a type like `Int32`.
    Type,
    /// An expression with a prefix operator.
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Identifier,
}

/// An expression with a prefix operator.
#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub operator: Token,
    pub operand:  Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub then_arm:  Box<Expression>,
    pub else_arm:  Option<Box<Expression>>,
}

/// An expression with a infix operator.
#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub left:  Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

/// An expression with two child expressions and an operator in between.
#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub operator: Token,
    pub left:     Box<Expression>,
    pub right:    Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    /// An expression that evaluates to a value.
    Value(Box<Expression>),
    /// A series of patterns separated by commas.
    Tuple(Vec<Pattern>),
    /// A named series of patterns separated by commas.
    Record(BTreeMap<String, Pattern>),
    /// A variable identifier with optional name and type.
    Variable(VariablePatternName, VariablePatternType),
}