use crate::parser::{Parser, ParserResult, ParserError, PrefixParselet};
use crate::types::{Expression, ExpressionKind, Pattern, Token, TokenKind};

#[derive(Debug, Clone)]
/// A parselet which parses an expression enclosed in parentheses.
pub struct TuplePatternParselet;

impl PrefixParselet for TuplePatternParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let mut children = vec![
            parser.parse_expression(0)?,
        ];

        if !parser.eof() {
            parser.advance();

            while !parser.eof() {
                match parser.peek().kind {
                    TokenKind::RightParen => {
                        parser.advance();
                        break
                    },

                    TokenKind::Comma => {
                        parser.advance();
                    },
    
                    _ => {
                        children.push(parser.parse_expression(0)?)
                    }
                }
            }
        } else {
            return Err(ParserError::UnexpectedEOF)
        }

        Ok(Expression {
            kind: ExpressionKind::Pattern(Pattern::Tuple {
                children,
            }),
            start_pos: 0,
            end_pos: 0,
            lexeme: format!("{}", token.lexeme),
        })
    }
}