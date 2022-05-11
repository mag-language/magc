//! Parse a named pattern and return it as a single entry of a key-value association.

use crate::parser::{
    Parser,
    ParserResult,
    ParserError,
    InfixParselet,
    PREC_RECORD,
};

use crate::types::{
    Expression,
    ExpressionKind,
    Pattern,
    VariablePattern,
    FieldPattern,
    Token,
    TokenKind,
};

#[derive(Debug, Clone)]
/// A named pattern, like `repeats: 4` or `name: n String`.
pub struct FieldPatternParselet;

impl FieldPatternParselet {
    fn expect_variable_pattern(&self, expression: Box<Expression>) -> Result<VariablePattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(
                Pattern::Variable(variable_pattern)
            ) => Ok(variable_pattern),

            _ => Err(ParserError::ExpectedPattern),
        }
    }

    fn expect_pattern(&self, expression: Box<Expression>) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Err(ParserError::ExpectedPattern),
        }
    }
}

impl InfixParselet for FieldPatternParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.consume_expect(TokenKind::Colon)?;

        let right = Box::new(parser.parse_expression(self.get_precedence())?);

        let VariablePattern { name, type_id } = self.expect_variable_pattern(left)?;

        let n = match name {
            Some(v) => v,
            None    => "_".to_string(),
        };

        Ok(Expression {
            kind: ExpressionKind::Pattern(Pattern::Field(FieldPattern {
                name: n,
                value: Box::new(self.expect_pattern(right)?),
            })),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        PREC_RECORD
    }
}