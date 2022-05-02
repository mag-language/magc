use crate::types::{Token, Literal, Pattern};
use crate::type_system::Typed;
use crate::parser::{ParserError};

use std::collections::HashMap;

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
    /// Match this pattern with a reference signature while obeying the precedence rules for patterns.
    pub fn linearize(&self, reference: Pattern) -> Result<bool, ParserError> {
        Ok(match self {
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
        
        Ok(false)
    }

    fn linearize_field(&self, reference: Pattern, name: String, value: Box<Expression>) -> Result<bool, ParserError> {
        Ok(false)
    }

    fn linearize_variable(&self, reference: Pattern, name: Option<String>, type_id: Option<String>) -> Result<bool, ParserError> {
        Ok(false)
    }
}

/// An expression with a prefix operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prefix {
    pub operator: Token,
    pub operand:  Box<Expression>,
}

/// An expression which defines a multimethod.
///
/// A method can be registered to the same name multiple times if the signature is not already
/// present. When it is called in the interpreter, we check if the call signature matches with one
/// of the defined method's signatures, and if it does, execute that function's body.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Method {
    /// The name by which this multimethod is referenced.
    pub name: String,
    /// The method signature which defines the arguments.
    pub signature: Box<Expression>,
    pub body: Box<Expression>,
}

/// An expression with a prefix operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Call {
    pub method: Box<Expression>,
    // The [`Record`] which contains the values of the arguments of the method call.
    pub signature:  Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub then_arm:  Box<Expression>,
    pub else_arm:  Option<Box<Expression>>,
}

/// An expression with a infix operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Infix {
    pub left:  Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

/// An expression with two child expressions and an operator in between.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpression {
    pub operator: Token,
    pub left:     Box<Expression>,
    pub right:    Box<Expression>,
}