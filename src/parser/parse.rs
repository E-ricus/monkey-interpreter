use super::ast::{Node, Program, Statement};
use crate::lexer::{Lexer, Token};

struct Parser<'a> {
    lex: &'a mut Lexer<'a>,
    curr_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lex: &'a mut Lexer<'a>) -> Self {
        let mut parser = Self {
            lex,
            curr_token: Token::Empty,
            peek_token: Token::Empty,
        };
        // Advance the token twice
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    fn parse_program(&mut self) -> Option<&Program> {
        unimplemented!("Parse program is not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let mut lex = Lexer::new(input);
        let parser = Parser::new(&mut lex);
        match parser.parse_program() {
            Some(program) => todo!(),
            None => ,
        }
    }
}
