use crate::parser::{
    Parser,
    ParserResult,
    ParserError,
    InfixParselet,
    PREC_CALL,
};
use crate::types::{
    Expression,
    ExpressionKind,
    Call,
    Token,
    TokenKind,
    Pattern,
    ValuePattern,
    VariablePattern,
};

#[derive(Debug, Clone)]
/// Parse a call expression like `fib(32)`.
pub struct CallParselet;

impl CallParselet {
    fn pattern_or_value_pattern(&self, expression: Box<Expression>) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Ok(Pattern::Value(ValuePattern {
                expression,
            })),
        }
    }

    fn expect_typeless_variable_pattern(&self, expression: Box<Expression>) -> Result<String, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(
                Pattern::Variable(pattern)
            ) => {
                if let VariablePattern { name: Some(name), type_id: None } = pattern {
                    Ok(name)
                } else {
                    Err(ParserError::ExpectedPattern)
                }
            },

            _ => Err(ParserError::ExpectedPattern),
        }
    }
}

impl InfixParselet for CallParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        let name = self.expect_typeless_variable_pattern(left)?;
        parser.consume_expect(TokenKind::LeftParen)?;

        let t = parser.peek()?;

        match t.kind {
            TokenKind::RightParen => parser.consume_expect(TokenKind::RightParen)?,
            _ => {
                let expr = parser.parse_expression(0)?;

                parser.consume_expect(TokenKind::RightParen)?;

                return Ok(Expression {
                    kind: ExpressionKind::Call(Call {
                        name,
                        signature: Some(self.pattern_or_value_pattern(Box::new(expr))?),
                    }),
                    lexeme:    token.lexeme,
                    start_pos: token.start_pos,
                    end_pos:   token.end_pos,
                })
            },
        };

        Ok(Expression {
            kind: ExpressionKind::Call(Call {
                name,
                signature: None,
            }),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        PREC_CALL
    }
}