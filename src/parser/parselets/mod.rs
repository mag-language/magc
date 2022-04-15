use crate::parser::{Parser, TokenBuffer};
use crate::expression::{Expression, ExpressionKind};
use crate::token::Token;

pub trait PrefixParselet {
    fn parse<'a>(&self, buffer: &'a mut TokenBuffer, token: Token) -> ExpressionKind<'a>;
}

/// A parselet which converts an identifier token into an expression.
pub struct IdentifierParselet;

impl PrefixParselet for IdentifierParselet {
    fn parse<'a>(&self, buffer: &'a mut TokenBuffer, token: Token) -> ExpressionKind<'a> {
        ExpressionKind::Identifier(token.lexeme)
    }
}

// /// A parselet which converts a token and the following expression into a prefix expression.
/*pub struct PrefixOperatorParselet;

impl PrefixParselet for PrefixOperatorParselet {
    fn parse<'a>(&mut self, parser: &'a mut Parser, token: Token) -> ExpressionKind<'a> {
        ExpressionKind::Identifier(format!("{}", token))
    }
}*/