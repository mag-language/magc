use crate::parser::{Parser, ParserResult, PrefixParselet};
use crate::types::{
    Block, CaseArm, Expression, ExpressionKind, Keyword, MatchExpression, ParserError, Token,
    TokenKind,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct MatchParselet;

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

/// Parse a case arm body after `then`.
/// - Same line as `then` → one expression, no `end`.
/// - Next line → parse expressions until `end`, consume it, wrap in Block.
fn parse_case_body(parser: &mut Parser, then_line: usize) -> Result<Box<Expression>, ParserError> {
    if parser.eof() {
        return Err(ParserError::UnexpectedEOF);
    }
    let multiline = parser.peek()?.line > then_line;
    if multiline {
        let mut children = vec![];
        loop {
            if parser.eof() {
                return Err(ParserError::UnexpectedEOF);
            }
            if let TokenKind::Keyword(Keyword::End) = parser.peek()?.kind {
                parser.advance(); // consume `end`
                break;
            }
            children.push(parser.parse_expression(0)?);
        }
        Ok(Box::new(wrap_block(children)))
    } else {
        Ok(Box::new(parser.parse_expression(0)?))
    }
}

impl PrefixParselet for MatchParselet {
    fn parse(&self, parser: &mut Parser, _token: Token) -> ParserResult {
        let subject = Box::new(parser.parse_expression(0)?);

        let mut arms = vec![];
        let mut else_arm: Option<Box<Expression>> = None;

        loop {
            let next = parser.peek()?.kind.clone();

            match next {
                TokenKind::Keyword(Keyword::Case) => {
                    parser.advance();
                    let pattern = parser.parse_expression(0)?.pattern_or_value_pattern()?;
                    let then_token = parser.consume_expect(TokenKind::Keyword(Keyword::Then))?;
                    let body = parse_case_body(parser, then_token.line)?;
                    arms.push(CaseArm { pattern, body });
                }
                TokenKind::Keyword(Keyword::Else) => {
                    parser.advance();
                    else_arm = Some(Box::new(parser.parse_expression(0)?));
                    parser.consume_expect(TokenKind::Keyword(Keyword::End))?;
                    break;
                }
                TokenKind::Keyword(Keyword::End) => {
                    return Err(crate::types::ParserError::UnexpectedType {
                        expected: "else branch".to_string(),
                        found: Some("end".to_string()),
                    });
                }
                _ => {
                    return Err(crate::types::ParserError::UnexpectedType {
                        expected: "case or else".to_string(),
                        found: Some(format!("{:?}", next)),
                    });
                }
            }
        }

        let else_arm = else_arm.unwrap();

        Ok(Expression {
            kind: ExpressionKind::Match(MatchExpression {
                subject,
                arms,
                else_arm,
            }),
            start_pos: 0,
            end_pos: 0,
        })
    }
}
