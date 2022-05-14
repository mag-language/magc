//! Parse a named pattern and return it as a single entry of a key-value association.

use crate::parser::{
    Parser,
    ParserResult,
    ParserError,
    PrefixParselet,
};

use crate::types::{
    Expression,
    ExpressionKind,
    Pattern,
    PairPattern,
    Token,
    TokenKind,
};

#[derive(Debug, Clone)]
/// Parse a list of expressions enclosed in brackets, like `[1, 2, 3]`.
pub struct ListParselet;

impl PrefixParselet for ListParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let mut kind;

        if parser.peek()?.kind == TokenKind::RightBracket {
            kind = ExpressionKind::List(None);
        } else {
            kind = ExpressionKind::List(
                Some(
                    Box::new(parser.parse_expression(0)?)
                )
            );
            parser.consume_expect(TokenKind::RightBracket);
        }

        Ok(Expression {
            kind,
            start_pos: 0,
            end_pos: 0,
            lexeme: format!("{}", token.lexeme),
        })
    }
}