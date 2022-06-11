use crate::types::{Literal, Pattern, ValuePattern};
use crate::type_system::Typed;
use crate::parser::ParserError;

mod block;
mod conditional;
mod infix;
mod method;
mod prefix;

pub use self::block::Block;
pub use self::conditional::Conditional;
pub use self::infix::Infix;
pub use self::method::{Method, Call};
pub use self::prefix::Prefix;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExpressionKind {
    /// An `if` expression running different branches of code based on a given condition.
    Conditional(Conditional),
    /// A list of expressions enclosed in brackets, like `[1, 2, 3]`.
    ///
    /// The optional single child expression allows putting zero or more entries into
    /// the list, making use of the pair pattern if there is more than one expression.
    List(Option<Box<Expression>>),
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    /// A value, tuple, field or variable pattern.
    Pattern(Pattern),
    /// A reference to a type, like `Int32`.
    Type,
    /// An expression with a prefix operator.
    Prefix(Prefix),
    /// Two expressions with an infix operator in between.
    Infix(Infix),
    /// An invocation of a method, like `print("Hello, World!")`
    Call(Call),
    /// A definition of a method with a given name, signature and body.
    Method(Method),
    /// A first-class chunk of code that can be passed around as a value.
    Block(Block),
    Identifier,
}

impl Expression {
    pub fn pattern_or_value_pattern(&self) -> Result<Pattern, ParserError> {
        match self.kind.clone() {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Ok(Pattern::Value(ValuePattern {
                expression: Box::new(self.clone()),
            })),
        }
    }

    pub fn expect_pattern(&self) -> Result<Pattern, ParserError> {
        match self.kind.clone() {
            ExpressionKind::Pattern(pattern) => Ok(pattern),

            _ => Err(ParserError::ExpectedPattern),
        }
    }
}

impl Typed for Expression {
    fn get_type(&self) -> Option<String> {
        match &self.kind {
            ExpressionKind::Conditional(_)   => Some(String::from("ConditionalExpression")),
            ExpressionKind::List(_)          => Some(String::from("ListExpression")),
            ExpressionKind::Literal(literal) => literal.get_type(),
            ExpressionKind::Pattern(pattern) => pattern.get_type(),
            ExpressionKind::Type             => Some(self.lexeme.clone()),
            ExpressionKind::Prefix(_)        => Some(String::from("PrefixExpression")),
            ExpressionKind::Infix(_)         => Some(String::from("InfixExpression")),
            ExpressionKind::Call(_)          => Some(String::from("CallExpression")),
            ExpressionKind::Method(_)        => Some(String::from("MethodExpression")),
            ExpressionKind::Block(_)         => Some(String::from("BlockExpression")),
            ExpressionKind::Identifier       => Some(String::from("Identifier")),
        }
    }
}