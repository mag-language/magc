//! Translate a Mag source string into a linear sequence of tokens.
//!
//! This is the first stage in the compiler, which takes a string, splits it
//! into UTF-8 characters and consumes them one at a time to create a list of
//! tokens that exactly represent the code contained in the source string.

use crate::types::{Token, TokenKind, Keyword, Literal};

use unicode_segmentation::UnicodeSegmentation;

/// An object which translates a Magpie source string into a linear sequence of tokens.
pub struct Lexer<'a> {
    position: usize,
    /// Tracks which line the current token is in.
    current_line: usize,
    source: Vec<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        // Split our source string into UTF-8 graphemes.
        let source = source.graphemes(true).collect::<Vec<&'a str>>();

        Self {
            position: 0,
            current_line: 1,
            source,
        }
    }

    /// Convert the source string into a linear collection of tokens.
    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.eof() {

            // Fetch our character and set the starting point of the lexeme.
            let character = self.source[self.position];
            let start_pos = self.position;

            let kind = match character {
                "!" => self.single_token(TokenKind::Bang),
                ":" => self.single_token(TokenKind::Colon),
                "," => self.single_token(TokenKind::Comma),
                "." => self.single_token(TokenKind::Dot),
                "[" => self.single_token(TokenKind::LeftBracket),
                "(" => self.single_token(TokenKind::LeftParen),
                "%" => self.single_token(TokenKind::Percent),
                "?" => self.single_token(TokenKind::QuestionMark),
                ")" => self.single_token(TokenKind::RightParen),
                "]" => self.single_token(TokenKind::RightBracket),

                "+" => self.match_next("=", TokenKind::PlusEqual, TokenKind::Plus),
                "*" => self.match_next("=", TokenKind::StarEqual, TokenKind::Star),
                "-" => self.match_next("=", TokenKind::MinusEqual, TokenKind::Minus),
                "=" => self.match_next("=", TokenKind::EqualEqual, TokenKind::Equal),
                ">" => self.match_next("=", TokenKind::GreaterEqual, TokenKind::Greater),
                "<" => self.match_next("=", TokenKind::SmallerEqual, TokenKind::Smaller),
                
                "/" => {
                    if self.peek() == "/" {
                        self.parse_comment()
                    } else if self.peek() == "=" {
                        self.advance();
                        TokenKind::SlashEqual
                    } else {
                        self.advance();
                        TokenKind::Slash
                    }
                },

                // Skip meaningless whitespace
                " " | "\t" | "\r" | "\n" => {
                    self.advance();
                    continue
                },

                "0" 
                | "1"
                | "2"
                | "3"
                | "4"
                | "5"
                | "6"
                | "7"
                | "8"
                | "9" => self.parse_number(),

                "A" | "B" | "C" | "D" | "E"
                | "F" | "G" | "H" | "I" | "J" 
                | "K" | "L" | "M" | "N" | "O"
                | "P" | "Q" | "R" | "S" | "T"
                | "U" | "V" | "W" | "X" | "Y" | "Z"
                    => self.parse_type(),

                "\"" => self.parse_string(),
                
                
                _ => self.parse_identifier_or_keyword(&character),
            };

            let end_pos = self.position;

            tokens.push(Token {
                kind,
                
                start_pos,
                end_pos
            });
        }

        tokens
    }

    // A utility function that allows us to call the advance 
    // method when returning a single character token.
    fn single_token(&mut self, kind: TokenKind) -> TokenKind {
        self.advance();
        kind
    }

    fn parse_identifier_or_keyword(&mut self, character: &'a str) -> TokenKind {
        let mut string = String::from(character);

        self.advance();

        while !self.eof() {
            let character =  self.source[self.position];

            match character {
                "!" | ":" | "," | "." | "[" | "(" |
                "%" | "?" | ")" | "]" | "*" | "/" |
                "+" | "-" | "=" | "<" | ">" | "\"" | 
                "'" | "\n" | "\r" | "\t" | " " | "{" |
                "}" | "`" | "^" => break,

                _ => {
                    self.advance();
                    string.push_str(character);
                },
            }
        }

        match self.current_lexeme.as_str() {
            "and"       => TokenKind::Keyword(Keyword::And),
            "as"        => TokenKind::Keyword(Keyword::As),
            "catch"     => TokenKind::Keyword(Keyword::Catch),
            "case"      => TokenKind::Keyword(Keyword::Case),
            "const"     => TokenKind::Keyword(Keyword::Const),
            "def"       => TokenKind::Keyword(Keyword::Def),
            "do"        => TokenKind::Keyword(Keyword::Do),
            "else"      => TokenKind::Keyword(Keyword::Else),
            "end"       => TokenKind::Keyword(Keyword::End),
            "enum"      => TokenKind::Keyword(Keyword::Enum),
            "if"        => TokenKind::Keyword(Keyword::If),
            "import"    => TokenKind::Keyword(Keyword::Import),
            "interface" => TokenKind::Keyword(Keyword::Interface),
            "it"        => TokenKind::Keyword(Keyword::It),
            "for"       => TokenKind::Keyword(Keyword::For),
            "match"     => TokenKind::Keyword(Keyword::Match),
            "or"        => TokenKind::Keyword(Keyword::Or),
            "return"    => TokenKind::Keyword(Keyword::Return),
            "then"      => TokenKind::Keyword(Keyword::Then),
            "this"      => TokenKind::Keyword(Keyword::This),
            "var"       => TokenKind::Keyword(Keyword::Var),
            "with"      => TokenKind::Keyword(Keyword::With),
            "while"     => TokenKind::Keyword(Keyword::While),

            "true"      => TokenKind::Literal(Literal::Boolean),
            "false"     => TokenKind::Literal(Literal::Boolean),

            _ => TokenKind::Identifier,
        }
    }

    fn parse_comment(&mut self) -> TokenKind {
        let mut comment = String::from(self.source[self.position]);

        self.advance();

        // Start parsing the comment.
        while !self.eof() {
            let character =  self.source[self.position];

            match character {
                "\n" => break,
                _ => {
                    self.advance();
                    comment.push_str(character)
                },
            }
        }

        TokenKind::Comment
    }

    fn parse_type(&mut self) -> TokenKind {
        let mut type_string = String::from("");

        self.advance();

        // Start parsing the comment.
        while !self.eof() {
            let character =  self.source[self.position];

            match character {
                "A" | "B" | "C" | "D" | "E"
                | "F" | "G" | "H" | "I" | "J" 
                | "K" | "L" | "M" | "N" | "O"
                | "P" | "Q" | "R" | "S" | "T" | "U"
                | "V" | "W" | "X" | "Y" | "Z"
                | "a" | "b" | "c" | "d" | "e"
                | "f" | "g" | "h" | "i" | "j" 
                | "k" | "l" | "m" | "n" | "o"
                | "p" | "q" | "r" | "s" | "t"
                | "v" | "w" | "x" | "y" | "z"
                | "0" | "1" | "2" | "3" | "4"
                | "5" | "6" | "7" | "8" | "9"
                 => {
                    self.advance();
                    type_string.push_str(character);
                },

                _ => break,
            }
        }

        TokenKind::Type
    }

    fn parse_number(&mut self) -> TokenKind {
        let mut number_string = String::from(self.source[self.position]);

        self.advance();

        // Start parsing the number.
        while !self.eof() {
            let character =  self.source[self.position];

            match character {
                "0" 
                | "1"
                | "2"
                | "3"
                | "4"
                | "5"
                | "6"
                | "7"
                | "8"
                | "9"
                | "." => {
                    self.advance();
                    self.current_lexeme.push_str(&character);
                    number_string.push_str(character);
                },

                _ => {
                    break
                },
            }
        }

        if number_string.contains(".") {
            TokenKind::Literal(
                Literal::Float
            )
        } else {
            TokenKind::Literal(
                Literal::Int
            )
        }
    }

    fn parse_string(&mut self) -> TokenKind {
        self.advance();

        while !self.eof() {
            let character =  self.source[self.position];

            match character {
                "\"" => {
                    self.advance();
                    break
                },

                _ => {
                    self.advance();
                },
            }
        }

        TokenKind::Literal(Literal::String)
    }

    /// Advance the pointer by one if we're not at the end.
    fn advance(&mut self) {
        if !self.eof() {
            self.position += 1;
        }
    }

    fn match_next(&mut self, character: &'static str, then: TokenKind, otherwise: TokenKind) -> TokenKind {
        self.advance();

        if self.current() == character {
            self.advance();
            then
        } else {
            otherwise
        }
    }

    fn current(&self) -> &'a str {
        self.source[self.position]
    }
    
    fn peek(&self) -> &'a str {
        self.source[self.position + 1]
    }

    fn eof(&self) -> bool {
        self.position >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token, TokenKind, Literal};

    #[test]
    fn scan_comment() {
        let mut lexer = Lexer::new("// This is a single line comment.");

        assert_eq!(
            lexer.parse(),
            vec![Token {
                kind: TokenKind::Comment,
                start_pos: 0,
                end_pos: 33,
            }]
        );
    }

    #[test]
    fn scan_integer() {
        let mut lexer = Lexer::new("1453");

        assert_eq!(
            lexer.parse(),
            vec![Token {
                kind: TokenKind::Literal(Literal::Int),
                start_pos: 0,
                end_pos: 4,
            }]
        );
    }

    #[test]
    fn scan_float() {
        let mut lexer = Lexer::new("12.38475");

        assert_eq!(
            lexer.parse(),
            vec![Token {
                kind: TokenKind::Literal(Literal::Float),
                start_pos: 0,
                end_pos: 8,
            }]
        );
    }

    #[test]
    fn scan_type() {
        let mut lexer = Lexer::new("Int32");

        assert_eq!(
            lexer.parse(),
            vec![Token {
                kind: TokenKind::Type,
                start_pos: 0,
                end_pos: 5,
            }]
        );
    }
}