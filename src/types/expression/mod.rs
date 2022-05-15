use crate::types::{Literal, Pattern};
use crate::type_system::Typed;

mod conditional;
mod infix;
mod method;
mod prefix;

pub use self::conditional::Conditional;
pub use self::infix::Infix;
pub use self::method::{Method, Call};
pub use self::prefix::Prefix;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub lexeme: String,
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
    Block(Vec<Expression>),
    Identifier,
}

impl Typed for Expression {
    fn get_type(&self) -> Option<String> {
        match self.kind {
            ExpressionKind::Conditional(_) => Some(String::from("ConditionalExpression")),
            ExpressionKind::List(_)        => Some(String::from("ListExpression")),

            _ => unimplemented!(),
        }
    }
}