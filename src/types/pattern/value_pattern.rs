use crate::types::{Expression};

/// An expression that evaluates to a value.
pub struct ValuePattern {
    pub expression: Box<Expression>,
}