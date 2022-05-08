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
mod infix;
mod literal;
mod method;
mod field_pattern;
mod tuple_pattern;
mod variable_pattern;
mod pair;
mod prefix;

pub use self::block::*;
pub use self::call::*;
pub use self::conditional::*;
pub use self::infix::*;
pub use self::literal::*;
pub use self::field_pattern::*;
pub use self::tuple_pattern::*;
pub use self::variable_pattern::*;
pub use self::prefix::*;
pub use self::pair::*;
pub use self::method::*;

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult;
}

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult;
    fn get_precedence(&self) -> usize;
}