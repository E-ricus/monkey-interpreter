use crate::lexer::Lexer;
use crate::token::Token;
use std::io::{Error, Stdin, Stdout, Write};

const PROMPT: &str = ">>";

pub fn start(stdin: Stdin, mut stdout: Stdout) -> Result<(), Error> {
    loop {
        write!(&stdout, "{PROMPT}")?;

        stdout.flush()?;

        let mut input = String::new();

        stdin.read_line(&mut input)?;

        let mut lexer = Lexer::new(&input);

        loop {
            let token = lexer.next_token();
            if token == Token::Eof {
                break;
            }
            writeln!(&stdout, "Token: {token}")?;
        }
    }
}
