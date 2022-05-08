use crate::types::{Expression, Environment};
use crate::parser::ParserError;

pub mod field;
pub mod tuple;
pub mod value;
pub mod pair;
pub mod variable;

use self::field::*;
use self::tuple::*;
use self::value::*;
use self::pair::*;
use self::variable::*;

/// A pattern that can be matched with an [`Expression`] to enable complex flow control
/// and full destructuring pattern matching, which increases the flexibility and 
/// expressivity within the language by a great degree.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Pattern {
    /// A named pattern, like `repeats: 4` or `name: n String`.
    Field(FieldPattern),
    /// A pattern enclosed in parentheses.
    Tuple(TuplePattern),
    /// An expression that evaluates to a value.
    Value(ValuePattern),
    /// A variable identifier with an optional type annotation.
    Variable(VariablePattern),
    /// A pair of patterns separated by a comma.
    Pair(PairPattern),
}