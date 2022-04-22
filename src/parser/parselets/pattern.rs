use super::{
    Parser,
    PrefixParselet,
    Token,
    TokenKind,
    ParserResult,
    Expression,
    ExpressionKind,
    Pattern,
    InfixExpression,
    PREC_PREFIX,
};

#[derive(Debug, Clone)]
pub struct VariablePatternParselet;

impl PrefixParselet for VariablePatternParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let name;

        if token.lexeme == "_" {
            name = None;
        } else {
            name = Some(token.lexeme.clone());
        }

        if !parser.eof() {
            let next_token = parser.peek();
            println!("[P] peeking next, is {:?}", next_token);
            
            let pattern = match next_token.kind {
                TokenKind::Type => {
                    parser.advance();

                    Pattern::Variable {
                        name,
                        type_id: Some(next_token.lexeme),
                    }
                },

                _ => Pattern::Variable {
                    name,
                    type_id: None,
                }
            };

            Ok(Expression {
                kind:      ExpressionKind::Pattern(pattern),
                lexeme:    token.lexeme,
                start_pos: token.start_pos,
                end_pos:   token.end_pos,
            })
        } else {
            Ok(Expression {
                kind:      ExpressionKind::Pattern(Pattern::Variable {
                    name,
                    type_id: None,
                }),
                lexeme:    token.lexeme,
                start_pos: token.start_pos,
                end_pos:   token.end_pos,
            })
        }
    }
}