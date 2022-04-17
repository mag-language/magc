use crate::parser::{Parser, PREC_SUM, PREC_PREFIX};
use crate::token::{Token, TokenKind};

use crate::expression::{
    Expression,
    ExpressionKind,
    PrefixExpression,
    InfixExpression,
};

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Expression;
}

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> Expression;
    fn get_precedence(&self) -> usize;
}

/// A parselet which converts an identifier token into an expression.
pub struct IdentifierParselet;

impl PrefixParselet for IdentifierParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Expression {
        Expression {
            kind:      ExpressionKind::Identifier,
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        }
    }
}

pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Expression {
        let kind = match token.kind {
            TokenKind::Literal(literal) => ExpressionKind::Literal(literal),
            _ => unreachable!(),
        };

        Expression {
            kind,
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        }
    }
}

/// A parselet which converts a token and the following expression into a prefix expression.
pub struct PrefixOperatorParselet;

impl PrefixParselet for PrefixOperatorParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Expression {
        let operator = token.clone();
        // TODO: temporary unwrap until we have proper error handling here
        let expr     = parser.parse_expression(PREC_PREFIX).unwrap();

        Expression {
            kind: ExpressionKind::Prefix(PrefixExpression {
                operator,
                operand: Box::new(expr),
            }),
            start_pos: 0,
            end_pos: 0,
            lexeme: format!("{}", token.lexeme),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InfixOperatorParselet {
    pub precedence: usize,
}

impl InfixParselet for InfixOperatorParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> Expression {
        println!("[P] infix parselet, left: {:?}, token: {:?}", left, token.clone());
        parser.advance();
        // TODO: temporary unwrap until we have proper error handling here
        let right = parser.parse_expression(self.precedence).unwrap();

        println!("[P] infix parselet, right: {:?}", right);

        Expression {
            kind: ExpressionKind::Infix(InfixExpression {
                left,
                operator: token.clone(),
                right: Box::new(right),
            }),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        }
    }

    fn get_precedence(&self) -> usize {
        self.precedence
    }
}