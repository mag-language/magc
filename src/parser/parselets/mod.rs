use crate::parser::Parser;
use crate::expression::Expression;
use crate::token::Token;

pub trait PrefixParselet {
    fn parse<'a>(&mut self, parser: &'a mut Parser, token: Token) -> Expression<'a>;
}

/// A parselet which converts an identifier token into an expression.
pub struct IdentifierParselet;

impl PrefixParselet for IdentifierParselet {
    fn parse<'a>(&mut self, parser: &'a mut Parser, token: Token) -> Expression<'a> {
        Expression::Identifier(format!("{}", token))
    }
}

/// A parselet which converts a token and the following expression into a prefix expression.
pub struct PrefixOperatorParselet;

impl PrefixParselet for PrefixOperatorParselet {
    fn parse<'a>(&mut self, parser: &'a mut Parser, token: Token) -> Expression<'a> {
        Expression::Identifier(format!("{}", token))
    }
}