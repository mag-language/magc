use crate::token::{Token, TokenKind, Literal};

use unicode_segmentation::UnicodeSegmentation;

/// A scanner which turns Magpie source code into a sequence of tokens.
pub struct Scanner {
    position: usize,
    // This variable is used to accumulate the parsed characters of the current
    // structure into a string containing the entire lexeme.
    current_lexeme: String,
    source: Vec<&'static str>,
}

impl Scanner {
    pub fn new(source: &'static str) -> Self {
        // Split our source string into UTF-8 graphemes.
        let source = source.graphemes(true).collect::<Vec<&'static str>>();

        Self {
            position: 0,
            current_lexeme: String::from(""),
            source,
        }
    }

    /// Convert the source string into a linear collection of tokens.
    pub fn parse(&mut self) -> Vec<Token> {
        // Start with a blank slate for our new lexeme.
        self.current_lexeme = String::from("");

        // Collect all parsed tokens in a Vec.
        let mut tokens = vec![];

        let start_pos = self.position;
        let character = self.source[self.position];

        self.current_lexeme.push_str(&character);

        while !self.eof() {
            let kind = match character {
                "!" => TokenKind::Bang,
                ":" => TokenKind::Colon,
                "," => TokenKind::Comma,
                "." => TokenKind::Dot,
                "[" => TokenKind::LeftBracket,
                "(" => TokenKind::LeftParen,
                "%" => TokenKind::Percent,
                "?" => TokenKind::QuestionMark,
                ")" => TokenKind::RightParen,
                "]" => TokenKind::RightBracket,

                "+" => self.match_next("=", TokenKind::PlusEqual, TokenKind::Plus),
                "*" => self.match_next("=", TokenKind::StarEqual, TokenKind::Star),
                "-" => self.match_next("=", TokenKind::MinusEqual, TokenKind::Minus),
                "=" => self.match_next("=", TokenKind::EqualEqual, TokenKind::Equal),
                ">" => self.match_next("=", TokenKind::GreaterEqual, TokenKind::Greater),
                "<" => self.match_next("=", TokenKind::SmallerEqual, TokenKind::Smaller),
                
                "/" => {
                    if self.peek() == "/" {
                        self.advance();
                        self.advance();
                        self.parse_comment()
                    } else if self.peek() == "=" {
                        TokenKind::SlashEqual
                    } else {
                        TokenKind::Slash
                    }
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
                
                
                _ => TokenKind::QuestionMark,
            };

            self.advance();

            let end_pos = self.position;

            tokens.push(Token {
                kind,
                lexeme: self.current_lexeme.clone(),
                start_pos,
                end_pos
            });
        }

        tokens
    }

    fn parse_comment(&mut self) -> TokenKind {
        let mut comment = String::from("");

        // Start parsing the comment.
        while !self.eof() {
            let character =  self.source[self.position];
            self.current_lexeme.push_str(&character);

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

        // Start parsing the comment.
        while !self.eof() {
            let character =  self.source[self.position];
            self.current_lexeme.push_str(&character);

            match character {
                "A" | "B" | "C" | "D" | "E"
                | "F" | "G" | "H" | "I" | "J" 
                | "K" | "L" | "M" | "N" | "O"
                | "P" | "Q" | "R" | "S" | "T"
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
                    type_string.push_str(character)
                },

                _ => break,
            }
        }

        TokenKind::Type
    }

    fn parse_number(&mut self) -> TokenKind {
        let mut number_string = String::from("");

        // Start parsing the number.
        while !self.eof() {
            let character =  self.source[self.position];
            self.current_lexeme.push_str(&character);

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

    /// Advance the pointer by one if we're not at the end.
    fn advance(&mut self) {
        if !self.eof() {
            self.position += 1;
        }
    }

    fn match_next(&mut self, character: &'static str, then: TokenKind, otherwise: TokenKind) -> TokenKind {
        if self.peek() == character {
            then
        } else {
            self.advance();
            otherwise
        }
    }
    
    fn peek(&self) -> &'static str {
        self.source[self.position + 1]
    }

    fn eof(&self) -> bool {
        self.position >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, Token, TokenKind, Literal};

    #[test]
    fn scan_comment() {
        let mut scanner = Scanner::new("// This is a single line comment.");

        assert_eq!(
            scanner.parse(),
            vec![Token {
                kind: TokenKind::Comment(" This is a single line comment.".to_string()),
                start_pos: 0,
                end_pos: 33,
            }]
        );
    }

    #[test]
    fn scan_integer() {
        let mut scanner = Scanner::new("1453");

        assert_eq!(
            scanner.parse(),
            vec![Token {
                kind: TokenKind::Literal(Literal::Int(1453)),
                start_pos: 0,
                end_pos: 4,
            }]
        );
    }

    #[test]
    fn scan_float() {
        let mut scanner = Scanner::new("12.38475");

        assert_eq!(
            scanner.parse(),
            vec![Token {
                kind: TokenKind::Literal(Literal::Float(12.38475)),
                start_pos: 0,
                end_pos: 8,
            }]
        );
    }

    #[test]
    fn scan_type() {
        let mut scanner = Scanner::new("Int32");

        assert_eq!(
            scanner.parse(),
            vec![Token {
                kind: TokenKind::Type("Int32".to_string()),
                start_pos: 0,
                end_pos: 5,
            }]
        );
    }
}