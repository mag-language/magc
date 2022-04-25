use crate::parser::{Parser, ParserResult, ParserError, InfixParselet};
use crate::types::{Expression, ExpressionKind, Pattern, Token, TokenKind};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RecordPatternParselet;

impl InfixParselet for RecordPatternParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.consume_expect(TokenKind::Comma)?;

        let mut fields = HashMap::new();

        if let ExpressionKind::Pattern(Pattern::Field { name, value}) = left.kind {
            fields.insert(name, value);
        } else {
            return Err(ParserError::UnexpectedExpression {
                expected: ExpressionKind::Pattern(Pattern::Variable { name: None, type_id: None}),
                found:    *left,
            })
        }

        while !parser.eof() {
            let next_token = parser.peek()?;

            match next_token.kind {
                TokenKind::Identifier => {
                    parser.consume_expect(TokenKind::Identifier)?;
                    parser.consume_expect(TokenKind::Colon)?;
                    fields.insert(next_token.lexeme, Box::new(parser.parse_expression(8)?));
                },

                _ => {
                    break
                }
            }
        }

        Ok(Expression {
            kind: ExpressionKind::Pattern(Pattern::Record {
                fields,
            }),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        8
    }
}