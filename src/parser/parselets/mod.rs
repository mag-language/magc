use crate::parser::{Parser, ParserResult, ParserError, PREC_PREFIX};

use crate::types::{
    Token,
    TokenKind,
    Keyword,
    Expression,
    ExpressionKind,
    CallExpression,
    PrefixExpression,
    InfixExpression,
    ConditionalExpression,
    Pattern,
};

mod call;
mod conditional;
mod field;
mod infix;
mod literal;
mod pattern;
mod prefix;
mod record;
mod tuple;

pub use self::call::*;
pub use self::conditional::*;
pub use self::field::*;
pub use self::infix::*;
pub use self::literal::*;
pub use self::pattern::*;
pub use self::prefix::*;
pub use self::record::*;
pub use self::tuple::*;

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult;
}

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult;
    fn get_precedence(&self) -> usize;
}