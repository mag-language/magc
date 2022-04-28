use crate::parser::{Parser, ParserResult, ParserError, PrefixParselet};
use crate::types::{Expression, ExpressionKind, Pattern, Token, TokenKind};

#[derive(Debug, Clone)]
/// A parselet which parses an expression enclosed in parentheses.
pub struct TuplePatternParselet;

impl PrefixParselet for TuplePatternParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let expr = parser.parse_expression(0)?;
        parser.consume_expect(TokenKind::RightParen)?;

        Ok(expr)
    }
}