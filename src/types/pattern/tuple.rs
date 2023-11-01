use crate::types::Pattern;

/// A pattern enclosed in parentheses.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TuplePattern {
    pub child: Box<Pattern>,
}

impl TuplePattern {
    pub fn desugar(mut self) -> TuplePattern {
        self.child.desugar();

        TuplePattern {
            child: self.child,
        }
    }
}