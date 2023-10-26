use crate::types::{
    Expression,
    ExpressionKind,
};

use crate::parser::Parser;

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

pub type LinearizeResult = Result<HashMap<VariablePattern, Box<Expression>>, ParserError>;

/// A pattern that can be matched with an [`Expression`] to enable complex flow control
/// and full destructuring pattern matching, which increases the flexibility and 
/// expressivity within the language by a great degree.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Pattern {
    /// A named pattern, like `repeats: 4` or `name: n String`.
    Field(FieldPattern),
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
    /// Convert an expression into a pattern, if possible.
    fn _pattern_or_value_pattern(&self, expression: Box<Expression>) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Ok(Pattern::Value(ValuePattern {
                expression,
            })),
        }
    }

    pub fn expect_field(self) -> Result<FieldPattern, ParserError> {
        match self {
            Pattern::Field(pattern) => Ok(pattern),
            _ => Err(ParserError::UnexpectedPattern {
                expected: String::from("FieldPattern"),
                found:    self.get_type().unwrap_or(String::from("<dynamically typed>")),
            }),
        }
    }

    pub fn expect_tuple(self) -> Result<TuplePattern, ParserError> {
        match self {
            Pattern::Tuple(pattern) => Ok(pattern),
            _ => Err(ParserError::UnexpectedPattern {
                expected: String::from("TuplePattern"),
                found:    self.get_type().unwrap_or(String::from("<dynamically typed>")),
            }),
        }
    }

    pub fn expect_value(self) -> Result<ValuePattern, ParserError> {
        match self {
            Pattern::Value(pattern) => Ok(pattern),
            _ => Err(ParserError::UnexpectedPattern {
                expected: String::from("ValuePattern"),
                found:    self.get_type().unwrap_or(String::from("<dynamically typed>")),
            }),
        }
    }

    pub fn expect_variable(self) -> Result<VariablePattern, ParserError> {
        match self {
            Pattern::Variable(pattern) => Ok(pattern),
            _ => Err(ParserError::UnexpectedPattern {
                expected: String::from("VariablePattern"),
                found:    self.get_type().unwrap_or(String::from("<dynamically typed>")),
            }),
        }
    }

    pub fn expect_pair(self) -> Result<PairPattern, ParserError> {
        match self {
            Pattern::Pair(pattern) => Ok(pattern),
            _ => Err(ParserError::UnexpectedPattern {
                expected: String::from("PairPattern"),
                found:    self.get_type().unwrap_or(String::from("<dynamically typed>")),
            }),
        }
    }
}

impl Typed for Pattern {
    fn get_type(&self) -> Option<String> {
        match self {
            Pattern::Field(_) => Some(String::from("FieldPattern")),
            Pattern::Tuple(_) => Some(String::from("TuplePattern")),
            Pattern::Value(_) => Some(String::from("ValuePattern")),
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
    /// Compare this pattern with another and destructure any variables if it matches.
    ///
    /// This function is used to determine which multimethod implementation matches the
    /// arguments of a given call, applying precedence rules to ensure that the most specific
    /// patterns are chosen. In the ubiquitous Fibonacci example, it decides which of the
    /// multimethods actually get executed based on the input parameters:
    ///
    /// ```text
    /// def fib(0) 0
    /// #       |- high specificity
    /// def fib(1) 1
    ///
    /// def fib(n Int) fib(n - 1) + fib(n - 2)
    /// #       ^- low specificity
    /// ```
    pub fn linearize(&self, parser: &mut Parser, other: Pattern) -> LinearizeResult {
        match self {
            Pattern::Field(reference)    => self.linearize_field(parser, reference.clone(), other),
            Pattern::Tuple(reference)    => self.linearize_tuple(parser, reference.clone(), other),
            Pattern::Value(reference)    => self.linearize_value(parser, reference.clone(), other),
            Pattern::Variable(reference) => self.linearize_variable(reference.clone(), other),
            Pattern::Pair(reference)     => self.linearize_pair(parser, reference.clone(), other),
        }
    }

    pub fn matches_with(&self, parser: &mut Parser, other: Pattern) -> bool {
        match self.linearize(parser, other) {
            Ok(_)  => true,
            Err(_) => false,
        }
    }

    pub fn get_precedence(&self) -> usize {
        match self {
            Pattern::Value(_) => 2,
            _                 => 1,
        }
    }

    fn linearize_field(&self, parser: &mut Parser, reference: FieldPattern, other: Pattern) -> LinearizeResult {
        if let Pattern::Field(given) = other {
            if given.name != reference.name { return Err(ParserError::NoMatch) }

            given.value.linearize(parser, *reference.value)
        } else {
            Err(ParserError::NoMatch)
        }
    }

    fn linearize_tuple(&self, parser: &mut Parser, reference: TuplePattern, other: Pattern) -> LinearizeResult {
        if let Pattern::Tuple(TuplePattern { child: other_pattern }) = other {
            reference.child.linearize(parser, *other_pattern)
        } else {
            Err(ParserError::NoMatch)
        }
    }

    fn linearize_value(&self, parser: &mut Parser, reference: ValuePattern, other: Pattern) -> LinearizeResult {
        let reference_lexeme = parser.get_lexeme(
            reference.expression.start_pos,
            reference.expression.end_pos,
        )?;

        if let Pattern::Value(ValuePattern { expression }) = other {
            let given_lexeme = parser.get_lexeme(
                expression.start_pos,
                expression.end_pos,
            )?;

            if reference.expression.kind == expression.kind
                    && reference_lexeme == given_lexeme {
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
                variables.insert(VariablePattern { name: Some(name), type_id: None }, expression);
            } else {
                // TODO: add proper error handling here!
                return Err(ParserError::NoMatch)
            }
        }
        
        Ok(variables)
    }

    fn linearize_pair(&self, parser: &mut Parser, reference: PairPattern, other: Pattern) -> LinearizeResult {
        if let Pattern::Pair(PairPattern { left, right }) = other {
            let mut left_map = reference.left.linearize(parser, *left)?;
            let right_map = reference.right.linearize(parser, *right)?;

            left_map.extend(right_map);

            Ok(left_map)
        } else {
            Err(ParserError::NoMatch)
        }
    }
}