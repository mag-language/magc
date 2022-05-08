//! Parse a named pattern and return it as a single entry of a key-value association.

use crate::parser::{
    Parser,
    ParserResult,
    ParserError,
    InfixParselet,
    PREC_RECORD,
};

use crate::types::{
    Expression,
    ExpressionKind,
    Pattern,
    VariablePattern,
    Token,
    TokenKind,
};

#[derive(Debug, Clone)]
/// A named pattern, like `repeats: 4` or `name: n String`.
pub struct FieldPatternParselet;

impl FieldPatternParselet {
    fn expect_variable_pattern(expression: Box<Expression>) -> Result<VariablePattern, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(
                Pattern::Variable(variable_pattern)
            ) => Ok(variable_pattern),

            _ => Err(ParserError::UnexpectedExpression {
                expected: ExpressionKind::Pattern(Pattern::Variable(_)),
                found: expression,
            }),
        }
    }
}

impl InfixParselet for FieldPatternParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.consume_expect(TokenKind::Colon)?;

        let value = Box::new(parser.parse_expression(self.get_precedence())?);

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
    }

    fn get_precedence(&self) -> usize {
        PREC_RECORD
    }
}