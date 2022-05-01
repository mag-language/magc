use crate::parser::{Parser, ParserResult, ParserError, PrefixParselet};
use crate::types::{Expression, ExpressionKind, Method, Token, TokenKind, Keyword};

#[derive(Debug, Clone)]
/// Parse a multimethod definition like `def fib(n Int) fib(n - 1) + fib(n - 2)`
pub struct MethodParselet;

impl PrefixParselet for MethodParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        // We'll implement complex signatures with receivers, getters and setters later,
        // so we just parse a simple method signature for now.
        let identifier = parser.consume_expect(TokenKind::Identifier)?;
        parser.consume_expect(TokenKind::LeftParen)?;
        let signature = Box::new(parser.parse_expression(0)?);
        parser.consume_expect(TokenKind::RightParen)?;
        let body = Box::new(parser.parse_expression(0)?);


        Ok(Expression {
            kind: ExpressionKind::Method(Method {
                name: identifier.lexeme,
                signature,
                body,
            }),
            start_pos: 0,
            end_pos: 0,
            lexeme: format!("{}", token.lexeme),
        })
    }
}