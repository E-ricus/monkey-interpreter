use std::{iter::Peekable, str::Chars};

use crate::token::Token;

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        Self {
            input: "".chars().peekable(),
            position: Default::default(),
            read_position: Default::default(),
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
        let token = match self.ch {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            '\0' => Token::Eof,
            _ => Token::Illegal,
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
}
