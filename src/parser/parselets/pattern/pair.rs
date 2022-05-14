//! Parse a pair of patterns separated by a comma.
//!
//! This parselet defines a left-associative infix operator which enables comma-separated
//! lists. Since a value pattern simply contains an [`Expression`], the contents of this
//! structure can be pretty much anything.

use crate::parser::{
    Parser,
    ParserResult,
    ParserError,
    InfixParselet,
    PREC_PAIR,
};

use crate::types::{
    Expression,
    ExpressionKind,
    Pattern,
    PairPattern,
    ValuePattern,
    Token,
};

/// Parse a pair of patterns separated by a comma.
#[derive(Debug, Clone)]
pub struct PairParselet;

impl PairParselet {
    fn pattern_or_value_pattern(&self, expression: Box<Expression>) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Ok(Pattern::Value(ValuePattern {
                expression,
            })),
        }
    }

    fn expect_pattern(&self, expression: Box<Expression>) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Err(ParserError::ExpectedPattern),
        }
    }
}

impl InfixParselet for PairParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.advance();

        let right = parser.parse_expression(self.get_precedence())?;

        Ok(Expression {
            kind: ExpressionKind::Pattern(Pattern::Pair(PairPattern {
                left: Box::new(self.pattern_or_value_pattern(left)?),
                right: Box::new(self.pattern_or_value_pattern(Box::new(right))?),
            })),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        PREC_PAIR
    }
}