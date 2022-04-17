use crate::token::{Token, TokenKind, Literal};
use crate::expression::{Expression, ExpressionKind};

use parselets::{
    PrefixParselet,
    InfixParselet,
    PrefixOperatorParselet,
    InfixOperatorParselet,
    IdentifierParselet,
    LiteralParselet,
};

use std::collections::HashMap;

pub mod parselets;

/// A parser which turns a linear token stream into a tree of Mag expressions.
pub struct Parser {
    position: usize,
    prefix_parselets: HashMap<TokenKind, &'static dyn PrefixParselet>,
    infix_parselets:  HashMap<TokenKind, &'static dyn InfixParselet>,
    source: Vec<Token>,
}

impl Parser {
    pub fn new(source: Vec<Token>) -> Self {
        let mut prefix_parselets = HashMap::new();
        let mut infix_parselets  = HashMap::new();

        prefix_parselets.insert(TokenKind::Identifier, &IdentifierParselet as &dyn PrefixParselet);

        prefix_parselets.insert(TokenKind::Literal(Literal::Int),     &LiteralParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Literal(Literal::Float),   &LiteralParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Literal(Literal::Boolean), &LiteralParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Literal(Literal::String),  &LiteralParselet as &dyn PrefixParselet);

        prefix_parselets.insert(TokenKind::Bang,  &PrefixOperatorParselet as &dyn PrefixParselet);
        //prefix_parselets.insert(TokenKind::Plus,  &PrefixOperatorParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Minus, &PrefixOperatorParselet as &dyn PrefixParselet);

        infix_parselets.insert(TokenKind::Plus,  &InfixOperatorParselet as &dyn InfixParselet);

        Self {
            position: 0,
            prefix_parselets,
            infix_parselets,
            source,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut expressions = vec![];

        while !self.eof() {
            expressions.push(self.parse_expression()?);
        }

        Ok(expressions)
    }

    pub fn parse_expression(
        &mut self,
    ) -> Result<Expression, ParserError> {
        let token = self.consume();

        if let Some(prefix) = self.prefix_parselets.get(&token.kind) {
            let left = prefix.parse(self, token.clone());

            if (self.eof()) {
                return Ok(left)
            }

            let token = self.peek();

            if let Some(infix) = self.infix_parselets.get(&token.kind) {
                // Return the infix expression created by the parselet.
                Ok(infix.parse(self, Box::new(left), token))
            } else {
                // We're just parsing a prefix expression, so we're done here.
                Ok(left)
            }
        } else {
            Err(ParserError::MissingPrefixParselet)
        }
    }

    /// Consume a token and advance the pointer.
    fn consume(&mut self) -> Token {
        let token = self.source[self.position].clone();
        self.advance();

        token
    }

    /// Advance the pointer by one if we're not at the end.
    fn advance(&mut self) {
        if !self.eof() {
            self.position += 1;
        }
    }

    fn peek(&self) -> Token {
        println!("peeking at position {}, token {}", self.position, self.source[self.position].clone());
        self.source[self.position].clone()
    }

    fn eof(&self) -> bool {
        self.position == self.source.len()
    }
}

#[derive(Debug, Clone)]
pub enum ParserError {
    MissingPrefixParselet,
}