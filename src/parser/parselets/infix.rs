use crate::parser::{Parser, ParserResult, InfixParselet};

use crate::types::{
    Expression,
    ExpressionKind,
    Infix,
    Pattern,
    TuplePattern,
    Token,
    TokenKind,
};

/// Parse a binary operator expression like `1 + 2`.
#[derive(Debug, Clone)]
pub struct InfixOperatorParselet {
    pub precedence: usize,
}

impl InfixParselet for InfixOperatorParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.advance();

        let right = parser.parse_expression(self.precedence)?;

        Ok(Expression {
            kind: ExpressionKind::Infix(Infix {
                left,
                operator: token.clone(),
                right: Box::new(right),
            }),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        self.precedence
    }
}