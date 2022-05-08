use crate::types::{
    Environment,
    Expression,
    Pattern,
};

/// An expression that evaluates to a value.
#[derive(Debug, Clone, Eq, Hash)]
pub struct ValuePattern {
    pub expression: Box<Expression>,
}

/*
    impl Pattern for ValuePattern {
        fn match_with(&self, expression: Box<Expression>) -> Option<Environment> {
            if self.expression == expression {
                // Return an empty environment since we're not destructuring anything here.
                // This indicates that the match has succeeded.
                Some(HashMap::new())
            } else {
                // Since the expressions do not match, we simply return [`None`] here.
                None
            }
        }
    }
*/

