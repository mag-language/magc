use crate::types::{
    Pattern,
};

/// A single entity within a record, like `repeats: 4` or `name: n String`.
#[derive(Debug, Clone, Eq, Hash)]
pub struct FieldPattern {
    pub name:  String,
    pub value: Box<dyn Pattern>,
}