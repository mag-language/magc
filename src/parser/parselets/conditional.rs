use crate::parser::{Parser, ParserResult, PrefixParselet};
use crate::types::{Block, Conditional, Expression, ExpressionKind, Keyword, Token, TokenKind};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
/// Parse a conditional expression like `if condition then {expression} else {expression}`
pub struct ConditionalParselet;

/// Parse a sequence of expressions that terminates at `else` or `end` (multi-line arm body).
/// Returns the expressions and the terminating keyword token kind.
fn parse_arm_body(parser: &mut Parser) -> Result<(Vec<Expression>, TokenKind), crate::types::ParserError> {
    let mut children = vec![];
    loop {
        if parser.eof() {
            return Err(crate::types::ParserError::UnexpectedEOF);
        }
        match parser.peek()?.kind {
            TokenKind::Keyword(Keyword::Else) | TokenKind::Keyword(Keyword::End) => {
                let kind = parser.peek()?.kind;
                return Ok((children, kind));
            }
            _ => children.push(parser.parse_expression(0)?),
        }
    }
}

fn wrap_block(children: Vec<Expression>) -> Expression {
    Expression {
        kind: ExpressionKind::Block(Block {
            environment: BTreeMap::new(),
            children,
        }),
        start_pos: 0,
        end_pos: 0,
    }
}

impl PrefixParselet for ConditionalParselet {
    fn parse(&self, parser: &mut Parser, _token: Token) -> ParserResult {
        let condition = Box::new(parser.parse_expression(0)?);
        let then_token = parser.consume_expect(TokenKind::Keyword(Keyword::Then))?;

        // One-liner when the first body token is on the same line as `then`.
        let multiline = !parser.eof() && parser.peek()?.line > then_token.line;

        if multiline {
            let (then_children, terminator) = parse_arm_body(parser)?;
            let then_arm = Box::new(wrap_block(then_children));

            let else_arm = if let TokenKind::Keyword(Keyword::Else) = terminator {
                parser.advance(); // consume `else`
                let (else_children, _) = parse_arm_body(parser)?;
                Some(Box::new(wrap_block(else_children)))
            } else {
                None
            };

            parser.consume_expect(TokenKind::Keyword(Keyword::End))?;

            Ok(Expression {
                kind: ExpressionKind::Conditional(Conditional {
                    condition,
                    then_arm,
                    else_arm,
                }),
                start_pos: 0,
                end_pos: 0,
            })
        } else {
            // One-liner: parse exactly one expression per arm, no `end`.
            let then_arm = Box::new(parser.parse_expression(0)?);

            let else_arm = if !parser.eof() {
                if let TokenKind::Keyword(Keyword::Else) = parser.peek()?.kind {
                    parser.advance();
                    Some(Box::new(parser.parse_expression(0)?))
                } else {
                    None
                }
            } else {
                None
            };

            Ok(Expression {
                kind: ExpressionKind::Conditional(Conditional {
                    condition,
                    then_arm,
                    else_arm,
                }),
                start_pos: 0,
                end_pos: 0,
            })
        }
    }
}
