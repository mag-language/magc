use crate::parser::{Parser, ParserResult, PrefixParselet};

use crate::types::{
    Token,
    TokenKind,
    Expression,
    ExpressionKind,
};

/// Parse a literal expression like `"Mike"`, `27`, `3.141`, `true`, or `false`.
pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    fn parse(&self, _parser: &mut Parser, token: Token) -> ParserResult {
        let kind = match token.kind {
            TokenKind::Literal(literal) => ExpressionKind::Literal(literal),
            _ => unreachable!(),
        };

        Ok(Expression {
            kind,
            
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }
}