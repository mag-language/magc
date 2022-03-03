use super::token::Token;

use unicode_segmentation::UnicodeSegmentation;

/// A scanner which turns Magpie source code into a sequence of tokens.
pub struct Scanner {
    position: usize,
    source: Vec<&'static str>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            position: 0,
            // Split our source string into UTF-8 lexemes.
            source: source.graphemes(true).collect::<Vec<&str>>(),
        }
    }

    /// Convert the source string into a linear collection of tokens.
    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.eof() {
            let token = match self.source[self.position] {
                ":" => Token::Colon,
                "," => Token::Comma,
                "." => Token::Dot,
                "=" => self.match_next("=", Token::EqualEqual, Token::Equal),
                ">" => self.match_next("=", Token::GreaterEqual, Token::Greater),
                "<" => self.match_next("=", Token::SmallerEqual, Token::Smaller),
                "[" => Token::LeftBracket,
                "(" => Token::LeftParen,
                "-" => Token::Minus,
                "-" => self.match_next("=", Token::MinusEqual, Token::Minus),
                "%" => Token::Percent,
                "+" => self.match_next("=", Token::PlusEqual, Token::Plus),
                "?" => Token::QuestionMark,
                ")" => Token::RightParen,
                "]" => Token::RightBracket,
                "/" => {
                    let next_char = self.peek();

                    if next_char == "=" {
                        Token::SlashEqual
                    } else if next_char == '/' {
                        self.parse_comment()
                    } else {
                        Token::Slash
                    }
                },
                "(" => Token::LeftParen,
                "(" => Token::LeftParen,
                "(" => Token::LeftParen,
                
                
                _ => {},
            };

            self.advance();
        }

        tokens
    }

    fn parse_comment(&self) -> Token {

    }

    fn advance(&mut self) {
        if !self.eof() {
            self.position += 1;
        }
    }

    fn match_next(&mut self, character: &'static str, then: Token, otherwise: Token) -> Token {
        if self.peek() == character {
            then
        } else {
            self.advance();
            otherwise
        }
    }

    fn eof(&self) -> bool {
        self.position >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}