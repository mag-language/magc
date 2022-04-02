use crate::token::{Token, TokenKind};
use parselets::PrefixParselet;

use std::collections::HashMap;

pub mod parselets;

/// A parser which turns a linear token stream into a tree of Mag expressions.
pub struct Parser<'a> {
    position: usize,
    source: Vec<Token>,
    prefix_parselets: HashMap<TokenKind, &'a dyn PrefixParselet>,
}