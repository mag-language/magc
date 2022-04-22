use crate::parser::{Parser, ParserResult, PREC_SUM, PREC_PREFIX, PREC_CONDITIONAL};

use crate::types::{
    Token,
    TokenKind,
    Keyword,
    Expression,
    ExpressionKind,
    CallExpression,
    PrefixExpression,
    InfixExpression,
    ConditionalExpression,
    Pattern,
};

pub use self::pattern::VariablePatternParselet;

pub mod pattern;

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult;
}

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult;
    fn get_precedence(&self) -> usize;
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

        let right = parser.parse_expression(self.precedence)?;

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

#[derive(Debug, Clone)]
/// A parselet which parses an expression enclosed in parentheses.
pub struct GroupParselet;

impl PrefixParselet for GroupParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let expression = parser.parse_expression(0)?;
        parser.consume_expect(TokenKind::RightParen);

        Ok(expression)
    }
}

#[derive(Debug, Clone)]
/// A parselet which parses a conditional expression like `if condition then {expression} else {expression}`
pub struct ConditionalParselet;

impl PrefixParselet for ConditionalParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        println!("[P] parsing conditional");
        let condition = Box::new(parser.parse_expression(0)?);
        parser.consume_expect(TokenKind::Keyword(Keyword::Then));

        let then_arm = Box::new(parser.parse_expression(0)?);

        if parser.eof() {
            return Ok(Expression {
                kind: ExpressionKind::Conditional(ConditionalExpression {
                    condition,
                    then_arm,
                    else_arm: None,
                }),
                start_pos: 0,
                end_pos: 0,
                lexeme: format!("{}", token.lexeme),
            })
        }

        if let TokenKind::Keyword(Keyword::Else) = parser.peek().kind {
            parser.advance();

            let else_arm = Box::new(parser.parse_expression(0)?);

            Ok(Expression {
                kind: ExpressionKind::Conditional(ConditionalExpression {
                    condition,
                    then_arm,
                    else_arm: Some(else_arm),
                }),
                start_pos: 0,
                end_pos: 0,
                lexeme: format!("{}", token.lexeme),
            })
        } else {
            Ok(Expression {
                kind: ExpressionKind::Conditional(ConditionalExpression {
                    condition,
                    then_arm,
                    else_arm: None,
                }),
                start_pos: 0,
                end_pos: 0,
                lexeme: format!("{}", token.lexeme),
            })
        }
    }
}

#[derive(Debug, Clone)]
/// A parselet which parses a call expression like `method()`
pub struct CallParselet;

impl InfixParselet for CallParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        // We can just skip the next character since there must be an opening brace here.
        parser.advance();
        parser.consume_expect(TokenKind::RightParen)?;

        Ok(Expression {
            kind: ExpressionKind::Call(CallExpression {
                method: left,

            }),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        100
    }
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