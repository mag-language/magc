use crate::parser::Parser;
use crate::expression::Expression;
use crate::token::Token;

pub trait PrefixParselet {
    fn parse<'a>(&mut self, parser: &'a mut Parser, token: Token) -> Expression<'a>;
}