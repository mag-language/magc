//! Parse a member like `person.favoriteColor` into a method call.
//!
//! A member is transformed into a method call similar to `favoriteColor(person)`
//! while pattern matching ensures the right multimethod is executed for the given type.

use crate::parser::{Parser, ParserResult, InfixParselet};

use crate::types::{
    Expression,
    ExpressionKind,
    Infix,
    Token,
};

/// Parse a member like `person.favoriteColor` into a method call.
#[derive(Debug, Clone)]
pub struct MemberParselet;

impl InfixParselet for MemberParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.advance();

        let right = parser.parse_expression(self.precedence)?;

        Ok(Expression {
            kind: ExpressionKind::Infix(Infix {
                left,
                operator: token.clone(),
                right: Box::new(right),
            }),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        self.precedence
    }
}