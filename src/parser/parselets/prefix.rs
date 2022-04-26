use crate::parser::{Parser, ParserResult, PREC_UNARY, PrefixParselet};
use crate::types::{Expression, ExpressionKind, PrefixExpression, Token};

/// A parselet which converts a token and the following expression into a prefix expression.
pub struct PrefixOperatorParselet;

impl PrefixParselet for PrefixOperatorParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let operator = token.clone();
        // TODO: temporary unwrap until we have proper error handling here
        let expr     = parser.parse_expression(PREC_UNARY)?;

        Ok(Expression {
            kind: ExpressionKind::Prefix(PrefixExpression {
                operator,
                operand: Box::new(expr),
            }),
            start_pos: 0,
            end_pos: 0,
            lexeme: format!("{}", token.lexeme),
        })
    }
}