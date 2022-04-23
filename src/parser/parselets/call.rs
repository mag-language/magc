use crate::parser::{Parser, ParserResult, InfixParselet};
use crate::types::{Expression, ExpressionKind, CallExpression, Token, TokenKind};

#[derive(Debug, Clone)]
/// A parselet which parses a call expression like `method()`
pub struct CallParselet;

impl InfixParselet for CallParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        // We can just skip the next character since there must be an opening brace here.
        parser.advance();
        parser.consume_expect(TokenKind::RightParen)?;

        Ok(Expression {
            kind: ExpressionKind::Call(CallExpression {
                method: left,

            }),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        100
    }
}