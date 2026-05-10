use crate::parser::{Parser, ParserResult, PrefixParselet};
use crate::types::{
    CaseArm, Expression, ExpressionKind, Keyword, MatchExpression, Token, TokenKind,
};

#[derive(Debug, Clone)]
pub struct MatchParselet;

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
                    parser.consume_expect(TokenKind::Keyword(Keyword::Then))?;
                    let body = Box::new(parser.parse_expression(0)?);
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
