//! A pair of patterns separated by a comma.

use crate::types::Pattern;

/// A pair of patterns separated by a comma.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PairPattern {
    pub left: Box<Pattern>,
    pub right: Box<Pattern>,
}

impl PairPattern {
    pub fn desugar(mut self) -> PairPattern {
        self.left.desugar();
        self.right.desugar();

        PairPattern {
            left: self.left,
            right: self.right,
        }
    }
}

impl std::fmt::Display for PairPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.left, self.right)
    }
}