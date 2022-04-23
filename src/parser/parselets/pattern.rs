use super::{
    Parser,
    PrefixParselet,
    Token,
    TokenKind,
    ParserResult,
    Expression,
    ExpressionKind,
    Pattern,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;

    #[test]
    fn type_and_name() {
        let mut lexer = Lexer::new("name String");
        let tokens = lexer.parse();
        let mut parser = Parser::new(tokens);

        assert_eq!(
            Ok(Expression {
                kind:      ExpressionKind::Pattern(Pattern::Variable {
                    name: Some("name".to_string()),
                    type_id: Some("String".to_string()),
                }),
                lexeme:    "name".to_string(),
                start_pos: 0,
                end_pos:   4,
            }),
            parser.parse_expression(0)
        );
    }

    #[test]
    fn no_type_and_no_name() {
        let mut lexer = Lexer::new("_");
        let tokens = lexer.parse();
        let mut parser = Parser::new(tokens);

        assert_eq!(
            Ok(Expression {
                kind:      ExpressionKind::Pattern(Pattern::Variable {
                    name: None,
                    type_id: None,
                }),
                lexeme:    "_".to_string(),
                start_pos: 0,
                end_pos:   1,
            }),
            parser.parse_expression(0)
        );
    }

    #[test]
    fn type_but_no_name() {
        let mut lexer = Lexer::new("_ Int");
        let tokens = lexer.parse();
        let mut parser = Parser::new(tokens);

        assert_eq!(
            Ok(Expression {
                kind:      ExpressionKind::Pattern(Pattern::Variable {
                    name: None,
                    type_id: Some("Int".to_string()),
                }),
                lexeme:    "_".to_string(),
                start_pos: 0,
                end_pos:   1,
            }),
            parser.parse_expression(0)
        );
    }

    #[test]
    fn name_but_no_type() {
        let mut lexer = Lexer::new("lexer");
        let tokens = lexer.parse();
        let mut parser = Parser::new(tokens);

        assert_eq!(
            Ok(Expression {
                kind:      ExpressionKind::Pattern(Pattern::Variable {
                    name: Some("lexer".to_string()),
                    type_id: None,
                }),
                lexeme:    "lexer".to_string(),
                start_pos: 0,
                end_pos:   5,
            }),
            parser.parse_expression(0)
        );
    }
}