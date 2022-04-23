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
    PrefixOperatorParselet,
    InfixOperatorParselet,
    VariablePatternParselet,
    LiteralParselet,
    RecordPatternParselet,
    FieldParselet,
    TuplePatternParselet,
    ConditionalParselet,
};

use std::collections::HashMap;
use std::rc::Rc;

pub mod parselets;

pub type ParserResult = Result<Expression, ParserError>;

pub static PREC_ASSIGNMENT: usize = 1000;
pub static PREC_CONDITIONAL: usize = 200;
pub static PREC_SUM: usize = 300;
pub static PREC_PRODUCT: usize = 400;
pub static PREC_EXPONENT: usize = 500;
pub static PREC_PREFIX: usize = 600;
pub static PREC_POSTFIX: usize = 700;
pub static PREC_CALL: usize = 800;

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
        prefix_parselets.insert(TokenKind::LeftParen,  &TuplePatternParselet      as &dyn PrefixParselet);

        prefix_parselets.insert(TokenKind::Literal(Literal::Int),     &LiteralParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Literal(Literal::Float),   &LiteralParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Literal(Literal::Boolean), &LiteralParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Literal(Literal::String),  &LiteralParselet as &dyn PrefixParselet);

        prefix_parselets.insert(TokenKind::Keyword(Keyword::If), &ConditionalParselet as &dyn PrefixParselet);

        prefix_parselets.insert(TokenKind::Bang,  &PrefixOperatorParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Plus,  &PrefixOperatorParselet as &dyn PrefixParselet);
        prefix_parselets.insert(TokenKind::Minus, &PrefixOperatorParselet as &dyn PrefixParselet);

        infix_parselets.insert(TokenKind::Plus,       infix_operator(PREC_SUM));
        infix_parselets.insert(TokenKind::Minus,      infix_operator(PREC_SUM));
        infix_parselets.insert(TokenKind::Identifier, infix_operator(PREC_SUM));
        infix_parselets.insert(TokenKind::Star,       infix_operator(PREC_PRODUCT));
        infix_parselets.insert(TokenKind::Slash,      infix_operator(PREC_PRODUCT));
        infix_parselets.insert(TokenKind::EqualEqual, infix_operator(0));

        infix_parselets.insert(TokenKind::LeftParen,  Rc::new(CallParselet) as Rc<dyn InfixParselet>);
        infix_parselets.insert(TokenKind::Comma,  Rc::new(RecordPatternParselet) as Rc<dyn InfixParselet>);
        infix_parselets.insert(TokenKind::Colon,  Rc::new(FieldParselet) as Rc<dyn InfixParselet>);

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
            expressions.push(self.parse_expression(0)?);
        }

        Ok(expressions)
    }

    pub fn parse_expression(
        &mut self,
        precedence: usize,
    ) -> Result<Expression, ParserError> {
        let token = self.consume();

        if let Some(prefix) = self.prefix_parselets.get(&token.kind) {
            let mut left = prefix.parse(self, token.clone())?;

            if self.eof() {
                return Ok(left)
            }

            while !self.eof() && precedence < self.get_precedence() {
                let token = self.peek();

                if let Some(infix) = self.infix_parselets.get(&token.kind).cloned() {
                    left = infix.parse(self, Box::new(left.clone()), token)?;
                }
            }

            Ok(left)
        } else {
            return Err(ParserError::MissingPrefixParselet(token.clone().kind))
        }
    }

    fn get_precedence(&self) -> usize {
        if let Some(infix) = self.infix_parselets.get(&self.peek().kind) {
            infix.get_precedence()
        } else {
            0
        }
    }

    /// Consume a token and advance the pointer.
    fn consume(&mut self) -> Token {
        let token = self.source[self.position].clone();
        self.advance();

        token
    }

    /// Consume a token with the given TokenKind, or return error.
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

    /// Advance the pointer by one if we're not at the end.
    fn advance(&mut self) {
        if !self.eof() {
            self.position += 1;
        }
    }

    fn peek(&self) -> Token {
        self.source[self.position].clone()
    }

    fn eof(&self) -> bool {
        self.position == self.source.len()
    }
}

#[derive(Debug, Clone)]
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