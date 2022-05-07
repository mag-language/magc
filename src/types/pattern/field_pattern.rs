use crate::types::{Pattern};

/// A single entity within a record, like `repeats: 4` or `name: n String`.
pub struct FieldPattern {
    pub name:  String,
    pub value: Box<dyn Pattern>,
}