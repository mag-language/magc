//! A first-class chunk of code that can be passed around as a value.

use crate::parser::{Parser, ParserResult, PREC_UNARY, PrefixParselet};
use crate::types::{Expression, ExpressionKind, Prefix, Token, TokenKind, Keyword};

/// A parselet which converts a token and the following expression into a prefix expression.
pub struct BlockParselet;

impl PrefixParselet for BlockParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let mut expressions = vec![];

        while !parser.eof() {
            match parser.peek()?.kind {
                TokenKind::Keyword(Keyword::End) => {
                    parser.advance();
                    break
                },

                _ => expressions.push(parser.parse_expression(0)?),
            };
        }

        Ok(Expression {
            kind: ExpressionKind::Block(expressions),
            start_pos: 0,
            end_pos: 0,
            lexeme: format!("{}", token.lexeme),
        })
    }
}