use crate::types::Expression;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VarDeclaration {
    pub name: String,
    pub value: Box<Expression>,
}
