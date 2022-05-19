//! A first-class chunk of code that can be passed around as a value.

use crate::parser::{Parser, ParserResult, PrefixParselet};
use crate::types::{Expression, ExpressionKind, Block, Token, TokenKind, Keyword};

use std::collections::BTreeMap;

/// Parse a series of expressions contained in a pair of `do` and `end` keywords.
pub struct BlockParselet;

impl PrefixParselet for BlockParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let mut children = vec![];

        while !parser.eof() {
            match parser.peek()?.kind {
                TokenKind::Keyword(Keyword::End) => {
                    parser.advance();
                    break
                },

                _ => children.push(parser.parse_expression(0)?),
            };
        }

        Ok(Expression {
            kind: ExpressionKind::Block(Block {
                environment: BTreeMap::new(),
                children,
            }),
            start_pos: 0,
            end_pos: 0,
            lexeme: format!("{}", token.lexeme),
        })
    }
}