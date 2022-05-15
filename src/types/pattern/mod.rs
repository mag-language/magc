use crate::types::{
    Expression,
    Environment,
};

use crate::type_system::Typed;
use crate::parser::{ParserError};
use std::collections::HashMap;

mod field;
mod tuple;
mod value;
mod pair;
mod variable;

pub use self::field::*;
pub use self::tuple::*;
pub use self::value::*;
pub use self::pair::*;
pub use self::variable::*;

pub type LinearizeResult = Result<HashMap<String, Box<Expression>>, ParserError>;

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

impl Typed for Pattern {
    fn get_type(&self) -> Option<String> {
        match self {
            Pattern::Field(_)    => Some(String::from("FieldPattern")),
            Pattern::Tuple(_)    => Some(String::from("TuplePattern")),
            Pattern::Value(ValuePattern {
                expression,
            })    => expression.get_type(),
            Pattern::Variable(
                VariablePattern {
                    name: _,
                    type_id,
                }
            ) => type_id.clone(),
            Pattern::Pair(_)     => Some(String::from("PairPattern")),
        }
    }
}

impl Pattern {
    /// Compare this pattern with another and return any destructured variables.
    ///
    /// This function recursively calls itself and the respective pattern methods
    /// to evaluate whether a tree of patterns matches with another. A typeless
    /// variable matches any value pattern, for example.
    pub fn linearize(&self, other: Pattern) -> LinearizeResult {
        match self {
            Pattern::Field(pattern)    => self.linearize_field(pattern.clone(), other),
            Pattern::Tuple(pattern)    => self.linearize_tuple(pattern.clone(), other),
            Pattern::Value(pattern)    => self.linearize_value(pattern.clone(), other),
            Pattern::Variable(pattern) => self.linearize_variable(pattern.clone(), other),
            Pattern::Pair(pattern)     => self.linearize_pair(pattern.clone(), other),
        }
    }

    fn linearize_field(&self, reference: FieldPattern, other: Pattern) -> LinearizeResult {
        Ok(HashMap::new())
    }

    fn linearize_tuple(&self, reference: TuplePattern, other: Pattern) -> LinearizeResult {
        Ok(HashMap::new())
    }

    fn linearize_value(&self, reference: ValuePattern, other: Pattern) -> LinearizeResult {
        Ok(HashMap::new())
    }

    fn linearize_variable(&self, reference: VariablePattern, other: Pattern) -> LinearizeResult {
        Ok(HashMap::new())
    }

    fn linearize_pair(&self, reference: PairPattern, other: Pattern) -> LinearizeResult {
        Ok(HashMap::new())
    }
}