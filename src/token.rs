use self::Keyword::*;
use self::TokenKind::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
	Bang,
	Colon,
	Comma,
	Dot,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Identifier(String),
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

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub kind:   TokenKind,
	pub start_pos:   usize,
	pub end_pos:   usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let string = match &self.kind {
			Colon 		 => ":".to_string(),
			Comma		 => ",".to_string(),
			Dot 		 => ".".to_string(),
			Equal 		 => "=".to_string(),
			EqualEqual 	 => "==".to_string(),
			Greater 	 => ">".to_string(),
			GreaterEqual => ">=".to_string(),

			Identifier(name) => format!("{}", name),
			Keyword(keyword) => format!("{}", keyword),

			LeftBracket => "[".to_string(),
			LeftParen 	=> "(".to_string(),

			Literal(literal) => format!("{}", literal),

			Minus 		 => "-".to_string(),
			MinusEqual 	 => "-=".to_string(),
			Percent 	 => "%".to_string(),
			Plus 		 => "+".to_string(),
			PlusEqual 	 => "+=".to_string(),
			QuestionMark => "?".to_string(),
			RightBracket => "]".to_string(),
			RightParen 	 => ")".to_string(),
			Slash 		 => "/".to_string(),
			SlashEqual 	 => "/=".to_string(),
			SlashSlash 	 => "//".to_string(),
			Smaller 	 => "<".to_string(),
			SmallerEqual => ">".to_string(),
			Star 		 => "*".to_string(),
			StarEqual 	 => "*=".to_string(),

			Type(type_string) => type_string.to_string(),
			Comment(comment) => comment.to_string(),
		};

        write!(f, "{}", string)
    }
}


#[derive(Debug, Clone, PartialEq)]
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

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let string = match self {
			And => "and",
			As => "as",
			Catch => "catch",
			Case => "case",
			Const => "const",
			Def => "def",
			Do => "do",
			Else => "else",
			End => "end",
			Enum => "enum",
			If => "if",
			Import => "import",
			It => "it",
			For => "for",
			Match => "match",
			Or => "or",
			Return => "return",
			Then => "then",
			This => "this",
			Var => "var",
			With => "with",
			While => "while",
		};

        write!(f, "{}", string)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
	Float(f64),
	String(String),
	Boolean(bool),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let string = match self {
			Literal::Int(int) 		  => format!("{}", int),
			Literal::Float(float) 	  => format!("{}", float),
			Literal::String(string)   => format!("{}", string),
			Literal::Boolean(boolean) => format!("{}", boolean),
		};

        write!(f, "{}", string)
    }
}