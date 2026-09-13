use crate::types::{Expression, ExpressionKind};

use crate::type_system::Typed;
use crate::types::ParserError;

mod pair;
mod record;
mod tuple;
mod value;
mod variable;

pub use self::pair::*;
pub use self::record::*;
pub use self::tuple::*;
pub use self::value::*;
pub use self::variable::*;

/// A pattern that can be matched with an [`Expression`] to enable complex flow control
/// and full destructuring pattern matching, which increases the flexibility and
/// expressivity within the language by a great degree.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Pattern {
    /// A named record field pattern, like `name: "Dave"` or `age: n Int`.
    Record(RecordPattern),
    /// A pattern enclosed in parentheses, like `(1 + 2)`
    Tuple(TuplePattern),
    /// Any expression that evaluates to a value, like `1 + 2` or `get_address_book()`.
    Value(ValuePattern),
    /// A variable identifier with an optional type annotation, such as `name` or `name String`.
    Variable(VariablePattern),
    /// A pair of patterns separated by a comma.
    Pair(PairPattern),
}

impl Pattern {
    fn _pattern_or_value_pattern(
        &self,
        expression: Box<Expression>,
    ) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),
            _ => Ok(Pattern::Value(ValuePattern { expression })),
        }
    }

    pub fn expect_record(self) -> Result<RecordPattern, ParserError> {
        match self {
            Pattern::Record(pattern) => Ok(pattern),
            _ => Err(ParserError::UnexpectedPattern {
                expected: String::from("RecordPattern"),
                found: self
                    .get_type()
                    .unwrap_or(String::from("<dynamically typed>")),
            }),
        }
    }

    pub fn expect_tuple(self) -> Result<TuplePattern, ParserError> {
        match self {
            Pattern::Tuple(pattern) => Ok(pattern),
            _ => Err(ParserError::UnexpectedPattern {
                expected: String::from("TuplePattern"),
                found: self
                    .get_type()
                    .unwrap_or(String::from("<dynamically typed>")),
            }),
        }
    }

    pub fn expect_value(self) -> Result<Expression, ParserError> {
        match self {
            Pattern::Value(pattern) => Ok(*pattern.expression),
            _ => Err(ParserError::UnexpectedPattern {
                expected: String::from("ValuePattern"),
                found: self
                    .get_type()
                    .unwrap_or(String::from("<dynamically typed>")),
            }),
        }
    }

    pub fn expect_variable(self) -> Result<VariablePattern, ParserError> {
        match self {
            Pattern::Variable(pattern) => Ok(pattern),
            _ => Err(ParserError::UnexpectedPattern {
                expected: String::from("VariablePattern"),
                found: self
                    .get_type()
                    .unwrap_or(String::from("<dynamically typed>")),
            }),
        }
    }

    pub fn expect_pair(self) -> Result<PairPattern, ParserError> {
        match self {
            Pattern::Pair(pattern) => Ok(pattern),
            _ => Err(ParserError::UnexpectedPattern {
                expected: String::from("PairPattern"),
                found: self
                    .get_type()
                    .unwrap_or(String::from("<dynamically typed>")),
            }),
        }
    }
}

impl Typed for Pattern {
    fn get_type(&self) -> Option<String> {
        match self {
            Pattern::Record(_) => Some(String::from("RecordPattern")),
            Pattern::Tuple(_) => Some(String::from("TuplePattern")),
            Pattern::Value(_) => Some(String::from("ValuePattern")),
            Pattern::Variable(_) => Some(String::from("VariablePattern")),
            Pattern::Pair(_) => Some(String::from("PairPattern")),
        }
    }
}

impl Pattern {
    pub fn desugar(&mut self) -> Self {
        match self {
            Pattern::Record(pattern) => Pattern::Record(pattern.clone().desugar()),
            Pattern::Tuple(pattern) => Pattern::Tuple(pattern.clone().desugar()),
            Pattern::Value(pattern) => Pattern::Value(pattern.clone().desugar()),
            Pattern::Variable(pattern) => Pattern::Variable(pattern.clone().desugar()),
            Pattern::Pair(pattern) => Pattern::Pair(pattern.clone().desugar()),
        }
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Pattern::Record(pattern) => write!(f, "{}", pattern),
            Pattern::Tuple(pattern) => write!(f, "{}", pattern),
            Pattern::Value(pattern) => write!(f, "{}", pattern),
            Pattern::Variable(pattern) => write!(f, "{}", pattern),
            Pattern::Pair(pattern) => write!(f, "{}", pattern),
        }
    }
}
