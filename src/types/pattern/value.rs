use crate::types::Expression;

/// An expression that evaluates to a value.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ValuePattern {
    pub expression: Box<Expression>,
}
