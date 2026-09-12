use crate::parser::{Parser, ParserError, ParserResult, PrefixParselet};
use crate::types::{Block, Expression, ExpressionKind, Keyword, Method, Pattern, Token, TokenKind, ValuePattern};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
/// Parse a multimethod definition like `def fib(n Int) fib(n - 1) + fib(n - 2)`
pub struct MethodParselet;

impl MethodParselet {
    fn pattern_or_value_pattern(
        &self,
        expression: Box<Expression>,
    ) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),
            _ => Ok(Pattern::Value(ValuePattern { expression })),
        }
    }

    fn parse_body(parser: &mut Parser, after_token_line: usize) -> Result<Box<Expression>, ParserError> {
        if parser.eof() {
            return Err(ParserError::UnexpectedEOF);
        }
        let multiline = parser.peek()?.line > after_token_line;

        if multiline {
            let mut children = vec![];
            loop {
                if parser.eof() {
                    return Err(ParserError::UnexpectedEOF);
                }
                if let TokenKind::Keyword(Keyword::End) = parser.peek()?.kind {
                    parser.advance();
                    break;
                }
                children.push(parser.parse_expression(0)?);
            }
            Ok(Box::new(Expression {
                kind: ExpressionKind::Block(Block {
                    environment: BTreeMap::new(),
                    children,
                }),
                start_pos: 0,
                end_pos: 0,
            }))
        } else {
            Ok(Box::new(parser.parse_expression(0)?))
        }
    }
}

impl PrefixParselet for MethodParselet {
    fn parse(&self, parser: &mut Parser, _token: Token) -> ParserResult {
        let method_name = parser.consume_expect(TokenKind::Identifier)?;
        parser.consume_expect(TokenKind::LeftParen)?;

        let kind = match parser.peek()?.kind {
            TokenKind::RightParen => {
                let right_paren = parser.consume_expect(TokenKind::RightParen)?;
                let body = Self::parse_body(parser, right_paren.line)?;
                ExpressionKind::Method(Method {
                    name: parser.get_lexeme(method_name.start_pos, method_name.end_pos)?,
                    signature: None,
                    body,
                })
            }
            _ => {
                let signature =
                    Some(self.pattern_or_value_pattern(Box::new(parser.parse_expression(0)?))?);
                let right_paren = parser.consume_expect(TokenKind::RightParen)?;
                let body = Self::parse_body(parser, right_paren.line)?;
                ExpressionKind::Method(Method {
                    name: parser.get_lexeme(method_name.start_pos, method_name.end_pos)?,
                    signature,
                    body,
                })
            }
        };

        Ok(Expression {
            kind,
            start_pos: 0,
            end_pos: 0,
        })
    }
}
