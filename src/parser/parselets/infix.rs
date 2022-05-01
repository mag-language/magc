use crate::parser::{Parser, ParserResult, InfixParselet};
use crate::types::{Expression, ExpressionKind, Infix, Pattern, Token, TokenKind};

#[derive(Debug, Clone)]
pub struct InfixOperatorParselet {
    pub precedence: usize,
}

impl InfixParselet for InfixOperatorParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.advance();

        let right = parser.parse_expression(self.precedence)?;

        match token.kind {
            TokenKind::Comma => Ok(Expression {
                kind: ExpressionKind::Pattern(Pattern::Tuple {
                    left,
                    right: Box::new(right),
                }),
                lexeme:    token.lexeme,
                start_pos: token.start_pos,
                end_pos:   token.end_pos,
            }),

            _ => Ok(Expression {
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
    }

    fn get_precedence(&self) -> usize {
        self.precedence
    }
}