//! Parse a single pattern enclosed in parentheses.

use crate::parser::{
    Parser,
    ParserResult,
    ParserError,
    PrefixParselet,
};

use crate::types::{
    Token,
    TokenKind,
    Expression,
    ExpressionKind,
    Pattern,
    TuplePattern,
};

/// Parse a single pattern enclosed in parentheses.
pub struct TuplePatternParselet;

impl TuplePatternParselet {
    fn expect_pattern(&self, expression: Expression) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Err(ParserError::ExpectedPattern),
        }
    }
}

impl PrefixParselet for TuplePatternParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let child = Box::new(self.expect_pattern(parser.parse_expression(0)?)?);
        parser.consume_expect(TokenKind::RightParen);

        Ok(Expression {
            kind: ExpressionKind::Pattern(
                Pattern::Tuple(TuplePattern { child }),
            ),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }
}