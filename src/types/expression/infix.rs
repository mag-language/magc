use crate::types::*;

/// An expression with a infix operator.
#[derive(Debug, Clone, Eq, Hash)]
pub struct Infix {
    pub left:  Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}