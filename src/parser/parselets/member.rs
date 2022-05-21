//! Parse a member like `person.favoriteColor` into a method call.
//!
//! A member is transformed into a method call similar to `favoriteColor(person)`
//! while pattern matching ensures the right multimethod is executed for the given type.

use crate::parser::{Parser, ParserResult, ParserError, InfixParselet, PREC_CALL};

use crate::types::{
    Expression,
    ExpressionKind,
    Pattern,
    VariablePattern,
    ValuePattern,
    Call,
    Token,
};

/// Parse a member like `person.favoriteColor` into a method call.
#[derive(Debug, Clone)]
pub struct MemberParselet;

impl MemberParselet {
    fn expect_typeless_variable_pattern(&self, expression: Box<Expression>) -> Result<Option<String>, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(
                Pattern::Variable(variable_pattern)
            ) => Ok(variable_pattern.name),

            _ => Err(ParserError::ExpectedPattern),
        }
    }
}

impl InfixParselet for MemberParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.advance();

        let name_opt = self.expect_typeless_variable_pattern(
            Box::new(parser.parse_expression(PREC_CALL)?)
        )?;

        let signature = Some(Pattern::Value(ValuePattern {
            expression: left,
        }));

        if let Some(name) = name_opt {
            Ok(Expression {
                kind: ExpressionKind::Call(Call {
                    name,
                    signature,
                }),
                lexeme:    token.lexeme,
                start_pos: token.start_pos,
                end_pos:   token.end_pos,
            })
        } else {
            Err(ParserError::ExpectedPattern)
        }
    }

    fn get_precedence(&self) -> usize {
        PREC_CALL
    }
}