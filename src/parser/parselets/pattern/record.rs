//! Parse a named record field pattern and return it as a key-value entry.

use crate::parser::{InfixParselet, Parser, ParserError, ParserResult, PREC_RECORD};

use crate::types::{
    Expression, ExpressionKind, Pattern, RecordPattern, Token, TokenKind, ValuePattern,
    VariablePattern,
};

#[derive(Debug, Clone)]
pub struct RecordPatternParselet;

impl RecordPatternParselet {
    fn expect_variable_pattern(
        &self,
        expression: Box<Expression>,
    ) -> Result<VariablePattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(Pattern::Variable(variable_pattern)) => Ok(variable_pattern),
            _ => Err(ParserError::ExpectedPattern),
        }
    }

    fn pattern_or_value_pattern(
        &self,
        expression: Box<Expression>,
    ) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),
            _ => Ok(Pattern::Value(ValuePattern { expression })),
        }
    }
}

impl InfixParselet for RecordPatternParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.consume_expect(TokenKind::Colon)?;

        let right = Box::new(parser.parse_expression(self.get_precedence())?);

        let VariablePattern { name, type_id: _ } = self.expect_variable_pattern(left)?;

        let n = match name {
            Some(v) => v,
            None => "_".to_string(),
        };

        Ok(Expression {
            kind: ExpressionKind::Pattern(Pattern::Record(RecordPattern {
                name: n,
                value: Box::new(self.pattern_or_value_pattern(right)?),
            })),

            start_pos: token.start_pos,
            end_pos: token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        PREC_RECORD
    }
}
