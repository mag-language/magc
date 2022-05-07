use crate::parser::{Parser, ParserResult, ParserError, PREC_UNARY};

use crate::types::{
    Token,
    TokenKind,
    Keyword,
    Expression,
    ExpressionKind,
    Call,
    Prefix,
    Infix,
    Conditional,
    Pattern,
};

mod block;
mod call;
mod conditional;
mod field;
mod group;
mod infix;
mod literal;
mod method;
mod variable_pattern;
mod prefix;

pub use self::block::*;
pub use self::call::*;
pub use self::conditional::*;
pub use self::field::*;
pub use self::group::*;
pub use self::infix::*;
pub use self::literal::*;
pub use self::variable_pattern::*;
pub use self::prefix::*;
pub use self::method::*;

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult;
}

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult;
    fn get_precedence(&self) -> usize;
}