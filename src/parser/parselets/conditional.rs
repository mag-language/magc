use crate::parser::{Parser, ParserResult, PrefixParselet};
use crate::types::{Expression, ExpressionKind, Conditional, Token, TokenKind, Keyword};

#[derive(Debug, Clone)]
/// Parse a conditional expression like `if condition then {expression} else {expression}`
pub struct ConditionalParselet;

impl PrefixParselet for ConditionalParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let condition = Box::new(parser.parse_expression(0)?);
        parser.consume_expect(TokenKind::Keyword(Keyword::Then))?;
        let then_arm = Box::new(parser.parse_expression(0)?);

        if !parser.eof() {
            if let TokenKind::Keyword(Keyword::Else) = parser.peek()?.kind {
                parser.advance();

                let else_arm = Box::new(parser.parse_expression(0)?);
                parser.consume_expect(TokenKind::Keyword(Keyword::End))?;

                Ok(Expression {
                    kind: ExpressionKind::Conditional(Conditional {
                        condition,
                        then_arm,
                        else_arm: Some(else_arm),
                    }),
                    start_pos: 0,
                    end_pos: 0,
                    
                })
            } else {
                parser.consume_expect(TokenKind::Keyword(Keyword::End))?;

                Ok(Expression {
                    kind: ExpressionKind::Conditional(Conditional {
                        condition,
                        then_arm,
                        else_arm: None,
                    }),
                    start_pos: 0,
                    end_pos: 0,
                    
                })
            }
        } else {
            Ok(Expression {
                kind: ExpressionKind::Conditional(Conditional {
                    condition,
                    then_arm,
                    else_arm: None,
                }),
                start_pos: 0,
                end_pos: 0,
                
            })
        }
    }
}