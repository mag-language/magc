use crate::types::{
    Pattern,
};

/// A named pattern, like `repeats: 4` or `name: n String`.
#[derive(Debug, Clone, Eq, Hash)]
pub struct FieldPattern {
    pub name:  String,
    pub value: Box<Pattern>,
}