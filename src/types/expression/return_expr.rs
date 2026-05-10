use crate::types::Expression;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ReturnExpression {
    pub value: Box<Expression>,
}
