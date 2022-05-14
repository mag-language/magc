use crate::parser::{Parser, ParserResult};

use crate::types::{
    Token,
    Expression,
};

mod block;
mod call;
mod conditional;
mod infix;
mod list;
mod literal;
mod method;
mod pattern;
mod prefix;

pub use self::block::*;
pub use self::call::*;
pub use self::conditional::*;
pub use self::infix::*;
pub use self::list::*;
pub use self::literal::*;
pub use self::pattern::*;
pub use self::prefix::*;
pub use self::method::*;

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult;
}

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult;
    fn get_precedence(&self) -> usize;
}