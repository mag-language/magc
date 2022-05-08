use crate::types::{Expression, Environment};
use crate::parser::ParserError;

use std::collections::HashMap;
use std::fmt::Debug;
use std::clone::Clone;
use std::cmp::Eq;
use std::hash::Hash;

pub mod field_pattern;
pub mod tuple_pattern;
pub mod value_pattern;
pub mod variable_pattern;

use self::field_pattern::*;
use self::tuple_pattern::*;
use self::value_pattern::*;
use self::variable_pattern::*;

/// A pattern that can be compared with an [`Expression`] to enable complex flow control
/// and full destructuring pattern matching, which increases the flexibility and 
/// expressivity within the language by a great degree.
pub enum Pattern {
    /// A single entity within a record, like `repeats: 4` or `name: n String`.
    Field(FieldPattern),
    /// A pattern enclosed in parentheses.
    Tuple(TuplePattern),
    /// An expression that evaluates to a value.
    Value(ValuePattern),
    /// A variable identifier with an optional type annotation.
    Variable(VariablePattern),
}