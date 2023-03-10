//! Parse a member like `person.favoriteColor` into a method call.
//!
//! A member is transformed into a method call similar to `favoriteColor(person)`
//! while pattern matching ensures the right multimethod is executed for the given type.

use crate::parser::{Parser, ParserResult, ParserError, InfixParselet, PREC_CALL};

use crate::types::{
    Expression,
    ExpressionKind,
    Pattern,
    Call,
    Token,
};

/// Parse a member like `person.favoriteColor` into a method call.
#[derive(Debug, Clone)]
pub struct MemberParselet;

impl MemberParselet {
    fn expect_typeless_variable_pattern(&self, expression: Box<Expression>) -> Result<Option<String>, ParserError> {
        match expression.kind {
            ExpressionKind::Pattern(
                Pattern::Variable(variable_pattern)
            ) => Ok(variable_pattern.name),

            _ => Err(ParserError::ExpectedPattern),
        }
    }
}

impl InfixParselet for MemberParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.advance();

        let right = parser.parse_expression(PREC_CALL)?;

        let name_opt = self.expect_typeless_variable_pattern(
            Box::new(right)
        )?;

        let signature = Some(left.expect_pattern()?);

        if let Some(name) = name_opt {
            Ok(Expression {
                kind: ExpressionKind::Call(Call {
                    name,
                    signature,
                }),
                
                start_pos: token.start_pos,
                end_pos:   token.end_pos,
            })
        } else {
            Err(ParserError::ExpectedPattern)
        }
    }

    fn get_precedence(&self) -> usize {
        PREC_CALL
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Expression, ExpressionKind, Call, Pattern, VariablePattern};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parses_getter_into_call() {
        let instance = "person";
        let member   = "favoriteColor";

        assert_eq!(
            Ok(vec![Expression {
                kind: ExpressionKind::Call(Call {
                    name: member.to_string(),
                    signature: Some(Pattern::Variable(VariablePattern {
                        name: Some(instance.to_string()),
                        type_id: None,
                    })),
                }),
                
                start_pos: 0,
                end_pos:   instance.len() + member.len(),
            }]),

            {
                let mut parser = Parser::new();
                let mut lexer  = Lexer::new();
                // We create this variable here so we can properly borrow the String as a &str
                let text = format!("{}.{}", instance, member);
                lexer.add_text(&text);

                parser.add_tokens(
                    crate::helpers::convert_to_graphemes("person.favoriteColor"),
                    lexer.parse(),
                );
                
                parser.parse()
            },
        );
    }
}