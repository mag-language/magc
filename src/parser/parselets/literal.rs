use crate::parser::{Parser, ParserResult, PrefixParselet};

use crate::types::{
    Token,
    TokenKind,
    Expression,
    ExpressionKind,
};

pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    fn parse(&self, _parser: &mut Parser, token: Token) -> ParserResult {
        let kind = match token.kind {
            TokenKind::Literal(literal) => ExpressionKind::Literal(literal),
            _ => unreachable!(),
        };

        Ok(Expression {
            kind,
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }
}