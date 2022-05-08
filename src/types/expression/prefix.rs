use crate::types::*;

/// An expression with a prefix operator.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Prefix {
    pub operator: Token,
    pub operand:  Box<Expression>,
}