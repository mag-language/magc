use crate::types::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
/// A first-class chunk of code that can be passed around as a value.
pub struct Block {
    pub environment: Environment,
    pub children:    Vec<Expression>,
}