use crate::types::{
    Pattern,
};

/// A pattern enclosed in parentheses.
#[derive(Debug, Clone, Eq, Hash)]
pub struct TuplePattern {
    pub child: Box<Pattern>,
}