use crate::parser::{Parser, ParserResult, PREC_SUM, PREC_PREFIX};
use crate::token::{Token, TokenKind};

use crate::expression::{
    Expression,
    ExpressionKind,
    PrefixExpression,
    InfixExpression,
};

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult;
}

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult;
    fn get_precedence(&self) -> usize;
}

/// A parselet which converts an identifier token into an expression.
pub struct IdentifierParselet;

impl PrefixParselet for IdentifierParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        Ok(Expression {
            kind:      ExpressionKind::Identifier,
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }
}

pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let kind = match token.kind {
            TokenKind::Literal(literal) => ExpressionKind::Literal(literal),
            _ => unreachable!(),
        };

        Ok(Expression {
            kind,
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }
}

/// A parselet which converts a token and the following expression into a prefix expression.
pub struct PrefixOperatorParselet;

impl PrefixParselet for PrefixOperatorParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let operator = token.clone();
        // TODO: temporary unwrap until we have proper error handling here
        let expr     = parser.parse_expression(PREC_PREFIX)?;

        Ok(Expression {
            kind: ExpressionKind::Prefix(PrefixExpression {
                operator,
                operand: Box::new(expr),
            }),
            start_pos: 0,
            end_pos: 0,
            lexeme: format!("{}", token.lexeme),
        })
    }
}

#[derive(Debug, Clone)]
pub struct InfixOperatorParselet {
    pub precedence: usize,
}

impl InfixParselet for InfixOperatorParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.advance();
        // TODO: temporary unwrap until we have proper error handling here
        let right = parser.parse_expression(self.precedence).unwrap();

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
        self.precedence
    }
}