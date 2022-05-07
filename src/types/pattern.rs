use crate::types::Expression;
use crate::parser::ParserError;

use std::collections::HashMap;

/*
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Pattern {
        /// An expression that evaluates to a value.
        Value {
            expr: Box<Expression>
        },

        /// A series of patterns separated by commas.
        ///
        /// The data structure is recursive since the comma is defined as an infix operator. This may
        /// look confusing at first, but is fairly easy to work with since you only need to call the
        /// method parsing the tuple items recursively.
        Tuple {
            left:  Box<Expression>,
            right: Box<Expression>,
        },

        /// A single entity within a record, like `repeats: 4` or `name: n String`.
        Field {
            name: String,
            value: Box<Expression>,
        },

        /// A variable identifier with optional name and type.
        Variable {
            name: Option<String>,
            type_id: Option<String>,
        },
    }
*/

/// A pattern that can be compared with an [`Expression`] to enable complex flow control
/// and full destructuring pattern matching, which increases the flexibility and 
/// expressivity within the language by a great degree.
pub trait Pattern {
    /// Return an environment containing the extracted, newly-bound variables if the
    /// match succeeds, or [`None`] if the expression doesn't match with this pattern.
    fn match_with(
        &self, 
        other: Box<Expression>,
    ) -> Option<HashMap<String, Box<Expression>>>;
}