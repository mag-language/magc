use crate::parser::{Parser, TokenBuffer};
use crate::expression::{Expression, ExpressionKind};
use crate::token::{Token, TokenKind};

pub trait PrefixParselet {
    fn parse<'a>(&self, buffer: &'a mut TokenBuffer, token: Token) -> Expression<'a>;
}

/// A parselet which converts an identifier token into an expression.
pub struct IdentifierParselet;

impl PrefixParselet for IdentifierParselet {
    fn parse<'a>(&self, buffer: &'a mut TokenBuffer, token: Token) -> Expression<'a> {
        Expression {
            kind:      ExpressionKind::Identifier,
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        }
    }
}

pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    fn parse<'a>(&self, buffer: &'a mut TokenBuffer, token: Token) -> Expression<'a> {
        let kind = match token.kind {
            TokenKind::Literal(literal) => ExpressionKind::Literal(literal),
            _ => unreachable!(),
        };

        Expression {
            kind,
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        }
    }
}

// /// A parselet which converts a token and the following expression into a prefix expression.
/*pub struct PrefixOperatorParselet;

impl PrefixParselet for PrefixOperatorParselet {
    fn parse<'a>(&mut self, parser: &'a mut Parser, token: Token) -> ExpressionKind<'a> {
        ExpressionKind::Identifier(format!("{}", token))
    }
}*/