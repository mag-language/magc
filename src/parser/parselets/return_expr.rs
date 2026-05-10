use crate::parser::{Parser, ParserResult, PrefixParselet};
use crate::types::{Expression, ExpressionKind, ReturnExpression, Token};

#[derive(Debug, Clone)]
pub struct ReturnParselet;

impl PrefixParselet for ReturnParselet {
    fn parse(&self, parser: &mut Parser, _token: Token) -> ParserResult {
        let value = Box::new(parser.parse_expression(0)?);

        Ok(Expression {
            kind: ExpressionKind::Return(ReturnExpression { value }),
            start_pos: 0,
            end_pos: 0,
        })
    }
}
