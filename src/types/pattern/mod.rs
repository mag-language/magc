use crate::types::{
    Expression,
    ExpressionKind,
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

impl Pattern {
    fn pattern_or_value_pattern(&self, expression: Box<Expression>) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Ok(Pattern::Value(ValuePattern {
                expression,
            })),
        }
    }
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
            Pattern::Field(reference)    => self.linearize_field(reference.clone(), other),
            Pattern::Tuple(reference)    => self.linearize_tuple(reference.clone(), other),
            Pattern::Value(reference)    => self.linearize_value(reference.clone(), other),
            Pattern::Variable(reference) => self.linearize_variable(reference.clone(), other),
            Pattern::Pair(reference)     => self.linearize_pair(reference.clone(), other),
        }
    }

    fn linearize_field(&self, reference: FieldPattern, other: Pattern) -> LinearizeResult {
        if let Pattern::Field(given) = other {
            if given.name != reference.name { return Err(ParserError::NoMatch) }

            given.value.linearize(*reference.value)
        } else {
            Err(ParserError::NoMatch)
        }
    }

    fn linearize_tuple(&self, reference: TuplePattern, other: Pattern) -> LinearizeResult {
        if let Pattern::Tuple(TuplePattern { child: other_pattern }) = other {
            reference.child.linearize(*other_pattern)
        } else {
            Err(ParserError::NoMatch)
        }
    }

    fn linearize_value(&self, reference: ValuePattern, other: Pattern) -> LinearizeResult {
        if let Pattern::Value(ValuePattern { expression }) = other {
            if reference.expression.kind == expression.kind
                    && reference.expression.lexeme == expression.lexeme {
                Ok(HashMap::new())
            } else {
                Err(ParserError::NoMatch)
            }
        } else {
            Err(ParserError::NoMatch)
        }
    }

    fn linearize_variable(&self, reference: VariablePattern, other: Pattern) -> LinearizeResult {
        let mut variables = HashMap::new();

        if let Some(name) = reference.name {
            // Extract value into environment and skip type checking for now.
            if let Pattern::Value(ValuePattern { expression }) = other {
                variables.insert(name, expression);
            } else {
                // TODO: add proper error handling here!
                return Err(ParserError::NoMatch)
            }
        }
        
        Ok(variables)
    }

    fn linearize_pair(&self, reference: PairPattern, other: Pattern) -> LinearizeResult {
        if let Pattern::Pair(PairPattern { left, right }) = other {
            let mut left_map = reference.left.linearize(*left)?;
            let right_map = reference.right.linearize(*right)?;

            left_map.extend(right_map);

            Ok(left_map)
        } else {
            Err(ParserError::NoMatch)
        }
    }
}