use crate::token::{Token, TokenKind};

use unicode_segmentation::UnicodeSegmentation;

/// A scanner which turns Magpie source code into a sequence of tokens.
pub struct Scanner {
    position: usize,
    source: Vec<&'static str>,
}

impl Scanner {
    pub fn new(source: &'static str) -> Self {
        // Split our source string into UTF-8 graphemes.
        let source = source.graphemes(true).collect::<Vec<&'static str>>();

        Self {
            position: 0,
            source,
        }
    }

    /// Convert the source string into a linear collection of tokens.
    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        let start_pos = self.position;

        while !self.eof() {
            let kind = match self.source[self.position] {
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
                
                
                _ => TokenKind::QuestionMark,
            };

            self.advance();

            let end_pos = self.position;

            tokens.push(Token {
                kind,
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

            match character {
                "\n" => break,
                _ => {
                    self.advance();
                    comment.push_str(character)
                },
            }
        }

        TokenKind::Comment(comment)
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
    use super::{Scanner, Token, TokenKind};

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
}