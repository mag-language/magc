use crate::types::{
    Keyword,
    Literal,
    Token, 
    TokenKind,
    Expression,
    ExpressionKind,
};

use parselets::{
    PrefixParselet,
    InfixParselet,
    CallParselet,
    BlockParselet,
    PrefixOperatorParselet,
    InfixOperatorParselet,
    VariablePatternParselet,
    LiteralParselet,
    RecordPatternParselet,
    MethodParselet,
    FieldParselet,
    TuplePatternParselet,
    ConditionalParselet,
};

use std::collections::HashMap;
use std::rc::Rc;

pub mod parselets;

pub type ParserResult = Result<Expression, ParserError>;

pub static PREC_ASSIGNMENT: usize = 10;
pub static PREC_RECORD: usize = 20;
pub static PREC_LOGICAL: usize = 30; // and or
pub static PREC_EQUALITY: usize = 40; // == !=
pub static PREC_COMPARISON: usize = 50; // < <= >= >
pub static PREC_TERM: usize = 60; // + -
pub static PREC_PRODUCT: usize = 70;
pub static PREC_EXPONENT: usize = 80;
pub static PREC_UNARY: usize = 90;
pub static PREC_CALL: usize = 100;

/// A parser which turns a linear token stream into a tree of Mag expressions.
pub struct Parser {
    position: usize,
    prefix_parselets: HashMap<TokenKind, &'static dyn PrefixParselet>,
    infix_parselets:  HashMap<TokenKind, Rc<dyn InfixParselet>>,
    source: Vec<Token>,
}

fn infix_operator(precedence: usize) -> Rc<dyn InfixParselet> {
    Rc::new(InfixOperatorParselet {
        precedence,
    }) as Rc<dyn InfixParselet>
}

impl Parser {
    pub fn new(source: Vec<Token>) -> Self {
        let mut prefix_parselets = HashMap::new();
        let mut infix_parselets  = HashMap::new();

        prefix_parselets.insert(TokenKind::Identifier, &VariablePatternParselet as &dyn PrefixParselet);
        //prefix_parselets.insert(TokenKind::LeftParen,  &TuplePatternParselet      as &dyn PrefixParselet);

        prefix_parselets.insert(TokenKind::Literal(Literal::Int),     &LiteralParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Literal(Literal::Float),   &LiteralParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Literal(Literal::Boolean), &LiteralParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Literal(Literal::String),  &LiteralParselet as &dyn PrefixParselet);

        prefix_parselets.insert(TokenKind::Keyword(Keyword::If),  &ConditionalParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Keyword(Keyword::Def), &MethodParselet      as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Keyword(Keyword::Do),  &BlockParselet      as &dyn PrefixParselet);

        prefix_parselets.insert(TokenKind::Bang,  &PrefixOperatorParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Plus,  &PrefixOperatorParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Minus, &PrefixOperatorParselet as &dyn PrefixParselet);

        infix_parselets.insert(TokenKind::Plus,       infix_operator(PREC_TERM));
        infix_parselets.insert(TokenKind::Minus,      infix_operator(PREC_TERM));
        infix_parselets.insert(TokenKind::Identifier, infix_operator(PREC_TERM));
        infix_parselets.insert(TokenKind::Star,       infix_operator(PREC_PRODUCT));
        infix_parselets.insert(TokenKind::Slash,      infix_operator(PREC_PRODUCT));
        infix_parselets.insert(TokenKind::EqualEqual, infix_operator(PREC_EQUALITY));
        infix_parselets.insert(TokenKind::Comma,      infix_operator(PREC_RECORD));

        infix_parselets.insert(TokenKind::LeftParen,  Rc::new(CallParselet) as Rc<dyn InfixParselet>);
        infix_parselets.insert(TokenKind::Colon,  Rc::new(FieldParselet) as Rc<dyn InfixParselet>);

        Self {
            position: 0,
            prefix_parselets,
            infix_parselets,
            source,
        }
    }

    /// Parse a series of expressions.
    pub fn parse(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut expressions = vec![];

        while !self.eof() {
            expressions.push(self.parse_expression(0)?);
        }

        Ok(expressions)
    }

    /// Parse a single expression with the given precedence.
    pub fn parse_expression(
        &mut self,
        precedence: usize,
    ) -> Result<Expression, ParserError> {
        let token = self.consume();

        // Let's see if we find a prefix parselet for the current token.
        if let Some(prefix) = self.prefix_parselets.get(&token.kind) {
            // Hand control over to our prefix parselet. This takes care of converting 
            // simple expressions like numbers, strings or variable identifiers.
            let mut left = prefix.parse(self, token.clone())?;

            if self.eof() {
                return Ok(left)
            }

            // This is the bit where real magic happens. This conditional check right here
            // is responsible for parsing infix expressions with the right precedence and
            // associativity so we can do math and generally have useful operators.
            while !self.eof() && precedence < self.get_precedence()? {
                let token = self.peek()?;

                // Hand control over to the infix parselet if there is one, and 
                // insert the previously parsed expression into this structure.
                if let Some(infix) = self.infix_parselets.get(&token.kind).cloned() {
                    left = infix.parse(self, Box::new(left.clone()), token)?;
                }
            }

            Ok(left)
        } else {
            return Err(ParserError::MissingPrefixParselet(token.clone().kind))
        }
    }

    /// Get the precedence for the current infix parselet.
    fn get_precedence(&self) -> Result<usize, ParserError> {
        if let Some(infix) = self.infix_parselets.get(&self.peek()?.kind) {
            Ok(infix.get_precedence())
        } else {
            Ok(0)
        }
    }

    /// Consume a token and advance the pointer.
    fn consume(&mut self) -> Token {
        let token = self.source[self.position].clone();
        self.advance();

        token
    }

    /// Consume a token with the given TokenKind, or return an error.
    fn consume_expect(&mut self, kind: TokenKind) -> Result<Token, ParserError> {
        let token = self.source[self.position].clone();

        if token.kind == kind {
            self.advance();

            Ok(token)
        } else {
            Err(ParserError::UnexpectedToken {
                expected: kind,
                found: token,
            })
        }
    }

    fn match_token(&self, kind: TokenKind) -> Result<bool, ParserError> {
        if !self.eof() {
            Ok(self.peek()?.kind == kind)
        } else {
            Err(ParserError::UnexpectedEOF)
        }
    }

    /// Advance the pointer by one if we're not at the end.
    fn advance(&mut self) {
        if !self.eof() {
            self.position += 1;
        }
    }

    fn peek(&self) -> Result<Token, ParserError> {
        if !self.eof() {
            Ok(self.source[self.position].clone())
        } else {
            Err(ParserError::UnexpectedEOF)
        }
    }

    fn eof(&self) -> bool {
        self.position == self.source.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    MissingPrefixParselet(TokenKind),
    UnexpectedToken {
        expected: TokenKind,
        found:    Token,
    },
    UnexpectedEOF,
    UnexpectedExpression {
        expected: ExpressionKind,
        found:    Expression,
    },
}