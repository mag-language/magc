#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TokenKind {
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
	Type(String),
    Comment(String),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Token {
	pub kind:   TokenKind,
	pub line:   usize,
	pub string: String,
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