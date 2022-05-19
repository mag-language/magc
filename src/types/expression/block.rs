use crate::types::*;

use std::collections::BTreeMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
/// A first-class chunk of code that can be passed around as a value.
pub struct Block {
    pub environment: BTreeMap<String, Expression>,
    pub children:    Vec<Expression>,
}