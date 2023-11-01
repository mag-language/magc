use super::{Expression, ExpressionKind, Token, TokenKind, Pattern};

pub type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, Clone)]
pub enum CompilerError {
    Generic(String),
    /// The given method signature has already been defined for this multimethod.
    DuplicateMethodSignature { signature: Option<Pattern> },
    ParserError(ParserError),
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_description: String = match self {
            Self::Generic(string) => string.clone(),
            Self::ParserError(error) => format!("{}", error),
            Self::DuplicateMethodSignature { .. }
                => format!("this method signature has already been defined for this multimethod"),
        };

        write!(f, "{}", error_description)
    }
}

impl From<ParserError> for CompilerError {
    fn from(e: ParserError) -> Self {
        CompilerError::ParserError(e)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    MissingPrefixParselet(TokenKind),
    UnexpectedToken {
        expected: TokenKind,
        found:    Token,
    },
    UnexpectedEOF,
    UnexpectedExpression {
        expected: ExpressionKind,
        found:    Expression,
    },
    UnexpectedType {
        expected: String,
        found:    Option<String>,
    },
    UnexpectedPattern {
        expected: String,
        found:    String,
    },
    ExpectedPattern,
    /// The linearization of the two given patterns failed.
    NoMatch,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_description = match self {
            Self::MissingPrefixParselet(kind)
                => format!("no prefix parselet found for token {:?}", kind),
            Self::UnexpectedToken { expected, found }
                => format!("expected token {:?}, found {:?}", expected, found),
            Self::UnexpectedEOF
                => format!("expected expression, found end of input"),
            Self::UnexpectedExpression { expected, found }
                => format!("expected expression {:?}, found {:?}", expected, found),
            Self::UnexpectedType { expected, found }
                => format!("expected type {:?}, found {:?}", expected, found),
            Self::UnexpectedPattern { expected, found }
                => format!("expected pattern {:?}, found {:?}", expected, found),
            Self::ExpectedPattern
                => format!("expected to find a pattern"),
            Self::NoMatch
                => format!("the given patterns do not match"),
            _ => format!("{:?}", self),
        };

        write!(f, "{}", error_description)
    }
}