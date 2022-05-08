//! Parse a pair of patterns separated by a comma.
//!
//! This parselet defines a left-associative infix operator which enables comma-separated
//! lists. Since a value pattern simply contains an [`Expression`], the contents of this
//! structure can be pretty much anything.

use crate::parser::{Parser, ParserResult, InfixParselet};

use crate::types::{
    Expression,
    ExpressionKind,
    Infix,
    Pattern,
    TuplePattern,
    Token,
    TokenKind,
};

/// Parse a pair of patterns separated by a comma.
#[derive(Debug, Clone)]
pub struct PairParselet;

impl InfixParselet for PairParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.advance();

        let right = parser.parse_expression(self.precedence)?;

        Ok(Expression {
            kind: ExpressionKind::Pattern(Pattern::Pair(PairPattern {
                left,
                right: Box::new(right),
            })),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        15
    }
}