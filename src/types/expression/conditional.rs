use crate::types::*;

#[derive(Debug, Clone, Eq, Hash)]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub then_arm:  Box<Expression>,
    pub else_arm:  Option<Box<Expression>>,
}