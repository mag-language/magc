use crate::parser::{Parser, ParserResult, PrefixParselet};
use crate::types::{Expression, ExpressionKind, Token, TokenKind, VarDeclaration};

#[derive(Debug, Clone)]
pub struct VarParselet;

impl PrefixParselet for VarParselet {
    fn parse(&self, parser: &mut Parser, _token: Token) -> ParserResult {
        let id_token = parser.consume_expect(TokenKind::Identifier)?;
        let name = parser.get_lexeme(id_token.start_pos, id_token.end_pos)?;
        parser.consume_expect(TokenKind::Equal)?;
        let value = Box::new(parser.parse_expression(0)?);

        Ok(Expression {
            kind: ExpressionKind::Var(VarDeclaration { name, value }),
            start_pos: 0,
            end_pos: 0,
        })
    }
}
