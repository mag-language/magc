//! Translate a Mag source string into a linear sequence of tokens.
//!
//! This is the first stage in the compiler, which takes a string, splits it
//! into UTF-8 characters and consumes them one at a time to create a list of
//! tokens that exactly represent the code contained in the source string.

use crate::types::{Token, TokenKind, Keyword, Literal};

use unicode_segmentation::UnicodeSegmentation;

/// An object which translates a Magpie source string into a linear sequence of tokens.
pub struct Lexer {
    position: usize,
    /// Tracks which line the current token is in.
    _current_line: usize,
    pub source: Vec<String>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            position: 0,
            _current_line: 1,
            source: vec![],
        }
    }

    pub fn add_text(&mut self, text: String) {
        self.source.append(
            &mut text.graphemes(true).map(String::from).collect::<Vec<String>>(),
        );
    }

    /// Convert the source string into a linear collection of tokens.
    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.eof() {

            // Fetch our character and set the starting point of the lexeme.
            let character = self.source[self.position].clone();
            let start_pos = self.position;

            let kind = match character.as_str() {
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
                
                
                _ => self.parse_identifier_or_keyword(character),
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

    fn parse_identifier_or_keyword(&mut self, mut character: String) -> TokenKind {
        self.advance();

        while !self.eof() {
            let c =  self.source[self.position].clone();

            match c.as_str() {
                "!" | ":" | "," | "." | "[" | "(" |
                "%" | "?" | ")" | "]" | "*" | "/" |
                "+" | "-" | "=" | "<" | ">" | "\"" | 
                "'" | "\n" | "\r" | "\t" | " " | "{" |
                "}" | "`" | "^" => break,

                _ => {
                    self.advance();
                    character = format!("{}{}", character, c);
                },
            }
        }

        match character.as_str() {
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
        let mut comment = String::from(self.source[self.position].clone());

        self.advance();

        // Start parsing the comment.
        while !self.eof() {
            let character =  self.source[self.position].clone();

            match character.as_str() {
                "\n" => break,
                _ => {
                    self.advance();
                    comment = format!("{}{}", comment, character);
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
            let character =  self.source[self.position].clone();

            match character.as_str() {
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
                    type_string = format!("{}{}", type_string, character);
                },

                _ => break,
            }
        }

        TokenKind::Type
    }

    fn parse_number(&mut self) -> TokenKind {
        let mut number_string = String::from(self.source[self.position].clone());

        self.advance();

        // Start parsing the number.
        while !self.eof() {
            let character =  self.source[self.position].clone();

            match character.as_str() {
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
                    number_string = format!("{}{}", number_string, character);
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
            let character =  self.source[self.position].clone();

            match character.as_str() {
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

    /// Get the string value of a literal from the source based on its start and end positions.
    pub fn get_literal_string(&self, start_pos: usize, end_pos: usize) -> Option<String> {
        if start_pos < self.source.len() && end_pos <= self.source.len() && start_pos <= end_pos {
            Some(self.source[start_pos..end_pos].concat())
        } else {
            None
        }
    }

    fn current(&self) -> String {
        self.source[self.position].clone()
    }
    
    fn peek(&self) -> String {
        self.source[self.position + 1].clone()
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
        let mut lexer = Lexer::new();
        lexer.add_text("// This is a single line comment.".to_string());

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
        let mut lexer = Lexer::new();
        lexer.add_text("1453".to_string());

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
        let mut lexer = Lexer::new();
        lexer.add_text("12.38475".to_string());

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
        let mut lexer = Lexer::new();
        lexer.add_text("Int32".to_string());

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