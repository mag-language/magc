use crate::parser::{
    Parser,
    ParserResult,
    ParserError,
    PrefixParselet,
};
use crate::types::{
    Expression,
    ExpressionKind,
    Method,
    Token,
    TokenKind,
    Pattern,
    ValuePattern,
};

#[derive(Debug, Clone)]
/// Parse a multimethod definition like `def fib(n Int) fib(n - 1) + fib(n - 2)`
pub struct MethodParselet;

impl MethodParselet {
    fn pattern_or_value_pattern(&self, expression: Box<Expression>) -> Result<Pattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Ok(Pattern::Value(ValuePattern {
                expression,
            })),
        }
    }
}

impl PrefixParselet for MethodParselet {
    fn parse(&self, parser: &mut Parser, _token: Token) -> ParserResult {
        // We'll implement complex signatures with receivers, getters and setters later,
        // so we just parse a simple method signature for now.
        let method_name = parser.consume_expect(TokenKind::Identifier)?;

        parser.consume_expect(TokenKind::LeftParen)?;

        let kind = match parser.peek()?.kind {
            // Empty method signature.
            TokenKind::RightParen => {
                parser.consume_expect(TokenKind::RightParen)?;

                let body = Box::new(parser.parse_expression(0)?);

                ExpressionKind::Method(Method {
                    name: parser.get_lexeme(method_name.start_pos, method_name.end_pos)?,
                    signature: None,
                    body,
                })
            },

            _ => {
                let signature = Some(self.pattern_or_value_pattern(
                    Box::new(parser.parse_expression(0)?)
                )?);

                parser.consume_expect(TokenKind::RightParen)?;

                let body = Box::new(parser.parse_expression(0)?);

                ExpressionKind::Method(Method {
                    name: parser.get_lexeme(method_name.start_pos, method_name.end_pos)?,
                    signature,
                    body,
                })
            }
        };

        Ok(Expression {
            kind,
            start_pos: 0,
            end_pos: 0,
            
        })
    }
}