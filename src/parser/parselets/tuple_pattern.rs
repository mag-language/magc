use crate::parser::{Parser, ParserResult, PrefixParselet};

use crate::types::{
    Token,
    TokenKind,
    Expression,
    ExpressionKind,
};

/// Parse multiple comma-separated patterns enclosed in parentheses.
pub struct TuplePatternParselet;

impl PrefixParselet for TuplePatternParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let content = parser.parse_expression(0)?;
        parser.consume_expect(TokenKind::RightParen);

        Ok(Expression {
            kind: ExpressionKind::Group(Box::new(content)),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }
}