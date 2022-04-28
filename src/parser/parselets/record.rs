use crate::parser::{Parser, ParserResult, ParserError, InfixParselet};
use crate::types::{Expression, ExpressionKind, Pattern, Token, TokenKind};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RecordPatternParselet;

impl InfixParselet for RecordPatternParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.consume_expect(TokenKind::Comma)?;

        let mut children = vec![*left.clone()];

        while !parser.eof() {
            match parser.peek()?.kind {
                TokenKind::Comma      => parser.advance(),
                TokenKind::RightParen => break,

                _                     => children.push(parser.parse_expression(0)?),
            }
        }

        Ok(Expression {
            kind: ExpressionKind::Pattern(Pattern::Record {
                children,
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