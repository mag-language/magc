use crate::type_system::Typed;

/// A single textual entity of a program like `(` or `if`.
///
/// The literal values contained in tokens are not parsed
/// until they're needed to avoid issues coming from the
/// interaction between hash maps and float values.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Token {
	// What type of token this object represents.
	pub kind:      TokenKind,
	pub start_pos: usize,
	pub end_pos:   usize,
	pub line: 	   usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TokenKind {
	Bang,
	Colon,
	Comma,
	Dot,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Identifier,
	Keyword(Keyword),
	LeftBracket,
	LeftParen,
	Literal(Literal),
	Minus,
	MinusEqual,
	Percent,
	Plus,
	PlusEqual,
	QuestionMark,
	RightBracket,
	RightParen,
	Slash,
	SlashEqual,
	SlashSlash,
	Smaller,
	SmallerEqual,
	Star,
	StarEqual,
	Type,
    Comment,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Keyword {
    And,
    As,
    Catch,
    Case,
    Const,
    Def,
    Do,
    Else,
    End,
    Enum,
    If, 
    Import,
	Interface,
    It,
    For,
    Match,
    Or,
    Return,
    Then,
    This,
    Var,
    With,
    While,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Literal {
    Int,
	Float,
	String,
	Boolean,
}

impl Typed for Literal {
    fn get_type(&self) -> Option<String> {
        Some(match self {
            Literal::Int   	 => String::from("Int"),
            Literal::Float 	 => String::from("Float"),
            Literal::String  => String::from("String"),
            Literal::Boolean => String::from("Boolean"),
        })
    }
}