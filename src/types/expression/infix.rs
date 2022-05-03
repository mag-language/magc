use crate::types::*;

/// An expression with a infix operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Infix {
    pub left:  Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}