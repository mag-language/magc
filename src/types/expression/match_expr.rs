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
    /// The mandatory else branch (required unless exhaustiveness can be statically proven).
    // TODO: skip requiring else when exhaustiveness can be statically proven,
    // e.g. all variants of an enum are covered.
    pub else_arm: Box<Expression>,
}
