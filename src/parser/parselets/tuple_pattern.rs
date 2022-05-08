use crate::parser::{
    Parser,
    ParserResult,
    ParserError,
    PrefixParselet,
}

use crate::types::{
    Token,
    TokenKind,
    Expression,
    ExpressionKind,
    Pattern,
}

/// Parse a pattern enclosed in parentheses.
pub struct TuplePatternParselet;

impl TuplePatternParselet {
    fn expect_pattern(expression: Box<Expression>) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Err(ParserError::UnexpectedExpression {
                expected: ExpressionKind::Pattern(_),
                found: expression,
            }),
        }
    }
}

impl PrefixParselet for TuplePatternParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let child = Box::new(self.expect_pattern(parser.parse_expression(0)?)?);
        parser.consume_expect(TokenKind::RightParen);

        Ok(Expression {
            kind: ExpressionKind::Pattern(
                Box::new(TuplePattern { child }) as Box<Pattern>,
            ),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }
}