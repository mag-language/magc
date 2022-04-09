use crate::token::{Token, TokenKind};
use crate::expression::Expression;
use parselets::PrefixParselet;

use std::collections::HashMap;

pub mod parselets;

/// A parser which turns a linear token stream into a tree of Mag expressions.
pub struct Parser {
    position: usize,
    source: Vec<Token>,
}

impl Parser {
    pub fn new(source: Vec<Token>) -> Self {
        Self {
            position: 0,
            source,
        }
    }

    pub fn parse_expression<'a>(
        &mut self, 
        mut prefix_parselets: HashMap<&'a TokenKind, &'a mut dyn PrefixParselet>
    ) -> Result<Expression, ParserError> {
        let token = self.source[self.position].clone();

        {
            if let Some(prefix) = prefix_parselets.get_mut(&token.kind) {
                Ok(prefix.parse(self, token.clone()))
            } else {
                Err(ParserError::MissingPrefixParselet)
            }
        }
    }

    /// Advance the pointer by one if we're not at the end.
    fn advance(&mut self) {
        if !self.eof() {
            self.position += 1;
        }
    }

    fn peek(&self) -> &Token {
        &self.source[self.position + 1]
    }

    fn eof(&self) -> bool {
        self.position >= self.source.len()
    }
}

pub enum ParserError {
    MissingPrefixParselet,
}

pub struct ParserBuffer {
    items: Vec<Token>,
    lexeme: Option<String>,
    position: usize,
}