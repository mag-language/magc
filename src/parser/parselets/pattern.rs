use super::{
    Parser,
    InfixParselet,
    Token,
    ParserResult,
    Expression,
    ExpressionKind,
    InfixExpression,
    PREC_PREFIX,
};

#[derive(Debug, Clone)]
pub struct PatternParselet {
    //pub precedence: usize,
}

impl InfixParselet for PatternParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.advance();

        let right = parser.parse_expression(0)?;

        Ok(Expression {
            kind: ExpressionKind::Infix(InfixExpression {
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
        0
    }
}