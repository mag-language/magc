use crate::parser::{Parser, ParserResult, InfixParselet};
use crate::types::{Expression, ExpressionKind, CallExpression, Token, TokenKind};

#[derive(Debug, Clone)]
/// A parselet which parses a call expression like `method()`
pub struct CallParselet;

impl InfixParselet for CallParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.consume_expect(TokenKind::LeftParen)?;

        let t = parser.peek();

        match t.kind {
            TokenKind::RightParen => parser.consume_expect(TokenKind::RightParen)?,
            _ => {
                let expr = parser.parse_expression(0)?;

                parser.consume_expect(TokenKind::RightParen)?;

                return Ok(Expression {
                    kind: ExpressionKind::Call(CallExpression {
                        method: left,
                        signature: None,
                    }),
                    lexeme:    token.lexeme,
                    start_pos: token.start_pos,
                    end_pos:   token.end_pos,
                })
            },
        };

        Ok(Expression {
            kind: ExpressionKind::Call(CallExpression {
                method: left,
                signature: None,
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