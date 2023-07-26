use std::{iter::Peekable, str::Chars};

use crate::token::Token;

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    // Unused for now
    // position: usize,
    // read_position: usize,
    ch: char,
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        Self {
            input: "".chars().peekable(),
            // Unused for now
            // position: Default::default(),
            // read_position: Default::default(),
            ch: Default::default(),
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lex = Lexer {
            input: input.chars().peekable(),
            ..Default::default()
        };
        lex.read_char();
        lex
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            'a'..='z' | 'A'..='Z' | '_' => {
                return Token::from(self.read_identifier());
            }
            '0'..='9' => {
                return Token::from(self.read_number());
            }
            '=' | '!' => {
                if self.peak_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let lit = format!("{}{}", ch, self.ch);
                    Token::from(lit)
                } else {
                    Token::from(self.ch)
                }
            }
            _ => Token::from(self.ch),
        };
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        self.ch = match self.input.peek() {
            Some(ch) => *ch,
            None => '\0',
        };

        self.input.next();
    }

    fn peak_char(&mut self) -> char {
        match self.input.peek() {
            Some(ch) => *ch,
            None => '\0',
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let 'a'..='z' | 'A'..='Z' | '_' = self.ch {
            identifier.push(self.ch);
            self.read_char();
        }
        identifier
    }

    fn read_number(&mut self) -> String {
        let mut identifier = String::new();
        while let '0'..='9' = self.ch {
            identifier.push(self.ch);
            self.read_char();
        }
        identifier
    }

    fn skip_whitespace(&mut self) {
        while let ' ' | '\t' | '\n' | '\r' = self.ch {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let expected_tokens: [Token; 9] = [
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(input);
        for tok in expected_tokens {
            let actual = lexer.next_token();
            assert_eq!(tok, actual)
        }
    }

    #[test]
    fn test_next_token_complete() {
        let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
     x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;";
        let expected_tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
            Token::GreatThan,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Int(String::from("10")),
            Token::Eq,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Int(String::from("10")),
            Token::NotEq,
            Token::Int(String::from("9")),
            Token::Semicolon,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(input);
        for (i, expected) in expected_tokens.into_iter().enumerate() {
            let actual = lexer.next_token();
            assert_eq!(
                expected, actual,
                "test #{i} - token type wrong. expected={expected:?}, got={actual:?}"
            )
        }
    }
}
