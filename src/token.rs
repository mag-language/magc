use self::Keyword::*;
use self::TokenKind::*;

/// A single textual entity of a program like `(` or `if`.
///
/// The literal values contained in tokens are not parsed
/// until they're needed to avoid issues coming from the
/// interaction between hash maps and float values.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Token {
	// What type of token this object represents.
	pub kind:   TokenKind,
	/// The string this token was parsed from.
	pub lexeme: String,
	pub start_pos:   usize,
	pub end_pos:   usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme)
    }
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
	Keyword,
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