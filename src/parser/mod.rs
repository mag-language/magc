use crate::token::{Token, TokenKind};
use crate::expression::{Expression, ExpressionKind};

use parselets::{
    PrefixParselet,
    IdentifierParselet,
};

use std::collections::HashMap;

pub mod parselets;

/// A parser which turns a linear token stream into a tree of Mag expressions.
pub struct Parser {
    position: usize,
    prefix_parselets: HashMap<TokenKind, &'static dyn PrefixParselet>,
    source: Vec<Token>,
}

impl Parser {
    pub fn new(source: Vec<Token>) -> Self {
        let mut prefix_parselets = HashMap::new();

        prefix_parselets.insert(TokenKind::Identifier, &IdentifierParselet as &dyn PrefixParselet);

        Self {
            position: 0,
            prefix_parselets,
            source,
        }
    }

    /*pub fn parse(&mut self) -> Result<Vec<Expression<'a>>, ParserError> {
        let mut expressions = vec![];
        let mut buffer = TokenBuffer::new(self.source.clone());

        let mut prefix_p = self.prefix_parselets.borrow_mut();

        prefix_p.insert(TokenKind::Identifier, &IdentifierParselet);

        while !self.eof() {
            {
                let start_pos = self.position;
                let kind = self.parse_expression(&mut buffer)?;
                let end_pos = self.position;

                expressions.push(Expression {
                    kind,
                    start_pos,
                    end_pos,
                })
            }
        }

        Ok(expressions)
    }*/

    pub fn parse_expression<'a>(
        &mut self,
        mut buffer: &'a mut TokenBuffer,
    ) -> Result<Expression<'a>, ParserError> {
        let token = self.source[self.position].clone();
        let start_pos = self.position;

        {
            println!("searching prefix parselet for token: {:?}", &token.kind);

            if let Some(parselet) = self.prefix_parselets.get(&token.kind) {
                let kind = parselet.parse(buffer, token);
                let end_pos = self.position;

                Ok(Expression {
                    kind,
                    start_pos,
                    end_pos,
                })
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

#[derive(Debug, Clone)]
pub enum ParserError {
    MissingPrefixParselet,
}

pub struct ParserBuffer {
    items: Vec<Token>,
    lexeme: Option<String>,
    position: usize,
}

/// A parser-specific buffer structure that can be passed around so
/// we don't need to reference the parser instance in the parselets.
pub struct TokenBuffer {
    pub source: Vec<Token>,
    pub position: usize,
}

impl TokenBuffer {
    pub fn new(source: Vec<Token>) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    /// Advance the pointer by one if we're not at the end.
    fn advance(&mut self) {
        if !self.eof() {
            self.position += 1;
        }
    }

    fn current(&self) -> Token {
        self.source[self.position].clone()
    }

    fn peek(&self) -> Token {
        self.source[self.position + 1].clone()
    }

    fn eof(&self) -> bool {
        self.position >= self.source.len()
    }
}