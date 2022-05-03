use crate::types::{Token, Literal, Pattern};
use crate::type_system::Typed;
use crate::parser::{ParserError};

use std::collections::HashMap;

mod conditional;
mod infix;
mod method;
mod prefix;

pub use self::conditional::Conditional;
pub use self::infix::Infix;
pub use self::method::{Method, Call};
pub use self::prefix::Prefix;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub lexeme: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionKind {
    Conditional(Conditional),
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    /// A value, tuple, record, field or variable pattern.
    Pattern(Pattern),
    /// A reference to a type like `Int32`.
    Type,
    /// An expression with a prefix operator.
    Prefix(Prefix),
    Infix(Infix),
    Call(Call),
    /// A definition of a multimethod.
    Method(Method),
    /// A first-class chunk of code that can be passed around as a value.
    Block(Vec<Expression>),
    /// One or more pattern enclosed in parentheses.
    Group(Box<Expression>),
    Identifier,
}


impl Expression {
    /// Match this expression to a reference signature while obeying the precedence rules for patterns.
    pub fn linearize(&self, reference: Pattern) -> Result<bool, ParserError> {
        Ok(match reference.clone() {
            Pattern::Value { expr }             => self.linearize_value(reference, expr.clone())?,
            Pattern::Tuple { left, right }      => self.linearize_tuple(reference, left.clone(), right.clone())?,
            Pattern::Field { name, value }      => self.linearize_field(reference, name.clone(), value.clone())?,
            Pattern::Variable { name, type_id } => self.linearize_variable(reference, name.clone(), type_id.clone())?,
        })
    }

    fn linearize_value(&self, reference: Pattern, given_expr: Box<Expression>) -> Result<bool, ParserError> {
        if let Pattern::Value { expr } = reference {
            Ok(expr == given_expr)
        } else {
            Ok(false)
        }
    }

    fn linearize_tuple(&self, reference: Pattern, left:  Box<Expression>, right: Box<Expression>) -> Result<bool, ParserError> {
        let mut does_match = false;

        does_match = left.linearize(reference);
        does_match = right.linearize(reference);

        Ok(does_match)
    }

    fn linearize_field(&self, reference: Pattern, name: String, value: Box<Expression>) -> Result<bool, ParserError> {
        Ok(false)
    }

    fn linearize_variable(&self, reference: Pattern, name: Option<String>, type_id: Option<String>) -> Result<bool, ParserError> {
        Ok(false)
    }
}