use crate::parser::{Parser, ParserResult, ParserError, InfixParselet, PREC_RECORD};
use crate::types::{Expression, ExpressionKind, Pattern, Token, TokenKind};

#[derive(Debug, Clone)]
pub struct FieldParselet;

impl InfixParselet for FieldParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.consume_expect(TokenKind::Colon)?;

        let value = Box::new(parser.parse_expression(self.get_precedence())?);

        if let ExpressionKind::Pattern(Pattern::Variable { name, type_id: _ }) = left.kind {
            if let Some(name) = name {
                Ok(Expression {
                    kind: ExpressionKind::Pattern(Pattern::Field {
                        name,
                        value,
                    }),
                    lexeme:    token.lexeme,
                    start_pos: token.start_pos,
                    end_pos:   token.end_pos,
                })
            } else {
                panic!("")
            }
        } else {
            Err(ParserError::UnexpectedExpression {
                expected: ExpressionKind::Pattern(Pattern::Variable {name: None, type_id: None}),
                found: *left,
            })
        }
    }

    fn get_precedence(&self) -> usize {
        PREC_RECORD
    }
}