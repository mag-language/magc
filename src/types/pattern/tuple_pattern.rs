use crate::types::{
    Environment,
    Expression,
    ExpressionKind,
    Pattern,
};

/// A pattern enclosed in parentheses.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TuplePattern {
    pub child: Box<dyn Pattern>,
}

impl Pattern for TuplePattern {
    fn match_with(&self, expression: Box<Expression>) -> Option<Environment> {
        None
    }
}