use crate::parser::{Parser, ParserResult, ParserError, PREC_SUM, PREC_PREFIX, PREC_CONDITIONAL};

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
use std::collections::HashMap;

pub mod pattern;
pub mod literal;

pub use self::literal::*;

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult;
}

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult;
    fn get_precedence(&self) -> usize;
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
pub struct TuplePatternParselet;

impl PrefixParselet for TuplePatternParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> ParserResult {
        let mut children = vec![
            parser.parse_expression(0)?,
        ];

        if !parser.eof() {
            parser.advance();

            while !parser.eof() {
                match parser.peek().kind {
                    TokenKind::RightParen => {
                        println!("[P] rparen");
                        parser.advance();
                        break
                    },

                    TokenKind::Comma => {
                        println!("[P] comma");
                        parser.advance();
                    },
    
                    _ => {
                        println!("[P] parsing expr");
                        children.push(parser.parse_expression(0)?)
                    }
                }
            }
        } else {
            return Err(ParserError::UnexpectedEOF)
        }

        Ok(Expression {
            kind: ExpressionKind::Pattern(Pattern::Tuple {
                children,
            }),
            start_pos: 0,
            end_pos: 0,
            lexeme: format!("{}", token.lexeme),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RecordPatternParselet;

impl InfixParselet for RecordPatternParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.consume_expect(TokenKind::Comma);

        let record_item = parser.parse_expression(8)?;

        Ok(Expression {
            kind: ExpressionKind::Pattern(Pattern::Record {
                children: vec![record_item],
            }),
            lexeme:    token.lexeme,
            start_pos: token.start_pos,
            end_pos:   token.end_pos,
        })
    }

    fn get_precedence(&self) -> usize {
        8
    }
}

#[derive(Debug, Clone)]
pub struct FieldParselet;

impl InfixParselet for FieldParselet {
    fn parse(&self, parser: &mut Parser, left: Box<Expression>, token: Token) -> ParserResult {
        parser.consume_expect(TokenKind::Colon);

        let value = Box::new(parser.parse_expression(8)?);

        if let ExpressionKind::Pattern(Pattern::Variable { name, type_id }) = left.kind {
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
        } else {
            Err(ParserError::UnexpectedExpression {
                expected: ExpressionKind::Pattern(Pattern::Variable {name: None, type_id: None}),
                found: *left,
            })
        }
    }

    fn get_precedence(&self) -> usize {
        8
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