use crate::types::{
    Expression,
};

use crate::parser::{
    ParserError,
};

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

impl Pattern {
    /// Match this pattern with a reference signature while obeying the precedence rules for patterns.
    pub fn linearize(&self, reference: Pattern) -> Result<bool, ParserError> {
        let mut does_match = false;

        does_match = match self {
            Pattern::Value { expr }             => self.linearize_value(reference, expr.clone())?,
            Pattern::Tuple { left, right }      => self.linearize_tuple(reference, left.clone(), right.clone())?,
            Pattern::Field { name, value }      => self.linearize_field(reference, name.clone(), value.clone())?,
            Pattern::Variable { name, type_id } => self.linearize_variable(reference, name.clone(), type_id.clone())?,
        };

        Ok(does_match)
    }

    fn linearize_value(&self, reference: Pattern, given_expr: Box<Expression>) -> Result<bool, ParserError> {
        if let Pattern::Value { expr } = reference {
            Ok(expr == given_expr)
        } else {
            Ok(false)
        }
    }

    fn linearize_tuple(&self, reference: Pattern, left:  Box<Expression>, right: Box<Expression>) -> Result<bool, ParserError> {
        Ok(false)
    }

    fn linearize_field(&self, reference: Pattern, name: String, value: Box<Expression>) -> Result<bool, ParserError> {
        Ok(false)
    }

    fn linearize_variable(&self, reference: Pattern, name: Option<String>, type_id: Option<String>) -> Result<bool, ParserError> {
        Ok(false)
    }
}