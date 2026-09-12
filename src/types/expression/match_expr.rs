use crate::types::Pattern;
use super::Expression;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CaseArm {
    pub pattern: Pattern,
    pub body: Box<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MatchExpression {
    pub subject: Box<Expression>,
    pub arms: Vec<CaseArm>,
    pub else_arm: Option<Box<Expression>>,
}
