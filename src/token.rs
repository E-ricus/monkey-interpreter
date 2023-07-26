#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Illegal,
    Eof,
    // Idntifiers + Literal
    Ident(String),
    Int(String),
    // Operators
    Assign,
    Plus,
    // Delimeters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // Keywords
    Funcion,
    Let,
}
