use crate::types::{Expression, Environment};
use crate::parser::ParserError;

use std::collections::HashMap;

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
pub trait Pattern {
    /// Return an environment containing the extracted, newly-bound variables if the
    /// match succeeds, or [`None`] if the expression doesn't match with this pattern.
    fn match_with(
        &self, 
        expression: Box<Expression>,
    ) -> Option<Environment>;
}