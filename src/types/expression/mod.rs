use crate::types::{
    Literal,
    Pattern,
    PairPattern,
    TokenKind,
    ValuePattern,
};
use crate::type_system::Typed;
use crate::types::ParserError;

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
    Type(String),
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

impl ExpressionKind {
    pub fn desugar(self) -> Self {
        match self {
            // Convert any infix expressions to method calls.
            ExpressionKind::Infix(mut infix) => {
                // Recursively desugar left and right expressions
                infix.left.desugar();
                infix.right.desugar();

                // Convert infix expressions to method calls
                let method_name = match &infix.operator.kind {
                    TokenKind::Plus => "+".to_string(),
                    TokenKind::Minus => "-".to_string(),
                    TokenKind::Star => "*".to_string(),
                    TokenKind::Slash => "/".to_string(),
                    _ => unimplemented!(),
                };
                ExpressionKind::Call(Call {
                    name: method_name,
                    signature: Some(Pattern::Pair(PairPattern {
                        left: Box::new(Pattern::Value(ValuePattern { expression: infix.left })),
                        right: Box::new(Pattern::Value(ValuePattern { expression: infix.right })),
                    })),
                })
            },

            ExpressionKind::Call(Call { ref name, ref signature }) => {
                if let Some(pattern) = signature {
                    match pattern {
                        Pattern::Value(ValuePattern { expression}) => {
                            let mut expr = expression.clone();
                            expr.desugar();
                            ExpressionKind::Call(Call {
                                name: name.to_string(),
                                signature: Some(Pattern::Value(ValuePattern {
                                    expression: expr,
                                })),
                            })
                        },
                        _ => unimplemented!(),
                    }
                } else {
                    self
                }
            },

            ExpressionKind::Method (mut method) => {
                ExpressionKind::Method(method)
            },
            // Desugar other expression kinds if necessary
            _ => self,
        }
    }
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

    /// Convert syntactic constructs into their actual semantics, like converting infix expressions
    /// to method calls.
    pub fn desugar(&mut self) {
        self.kind = self.kind.clone().desugar();
    }
}

impl Typed for Expression {
    fn get_type(&self) -> Option<String> {
        match &self.kind {
            ExpressionKind::Conditional(_)   => Some(String::from("ConditionalExpression")),
            ExpressionKind::List(_)          => Some(String::from("ListExpression")),
            ExpressionKind::Literal(literal) => literal.get_type(),
            ExpressionKind::Pattern(pattern) => pattern.get_type(),
            ExpressionKind::Type(type_id)    => Some(type_id.clone()),
            ExpressionKind::Prefix(_)        => Some(String::from("PrefixExpression")),
            ExpressionKind::Infix(_)         => Some(String::from("InfixExpression")),
            ExpressionKind::Call(_)          => Some(String::from("CallExpression")),
            ExpressionKind::Method(_)        => Some(String::from("MethodExpression")),
            ExpressionKind::Block(_)         => Some(String::from("BlockExpression")),
            ExpressionKind::Identifier       => Some(String::from("Identifier")),
        }
    }
}