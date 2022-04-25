use crate::parser::{Parser, ParserResult, ParserError, PrefixParselet};
use crate::types::{Expression, ExpressionKind, MethodExpression, Token, TokenKind, Keyword};

#[derive(Debug, Clone)]
/// A parselet which parses a multimethod definition like `def sayHello(name String)`
pub struct MethodParselet;

impl PrefixParselet for MethodParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        // We'll implement complex signatures with receivers, getters and setters later,
        // so we just parse a simple method signature for now.
        let identifier = parser.consume_expect(TokenKind::Identifier)?;
        let signature = Box::new(parser.parse_expression(0)?);
        let mut body = vec![];

        match parser.peek()?.kind {
            // Parse a block with a number of expressions in it.
            TokenKind::Keyword(Keyword::Do) => {
                while !parser.eof() {
                    match parser.peek()?.kind {
                        TokenKind::Keyword(Keyword::End) => {
                            parser.advance();
                            break
                        },

                        _ => {
                            body.push(parser.parse_expression(0)?);
                        }
                    }
                }
            },

            _ => {
                body.push(parser.parse_expression(0)?);
            }
        }


        Ok(Expression {
            kind: ExpressionKind::Method(MethodExpression {
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