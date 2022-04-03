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

impl<'a> Parser<'a> {
    pub fn new(source: Vec<Token>) -> Self {
        let prefix_parselets = HashMap::new();

        Self {
            position: 0,
            source,
            prefix_parselets,
        }
    }
}