use crate::types::*;

/// An expression with a prefix operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prefix {
    pub operator: Token,
    pub operand:  Box<Expression>,
}