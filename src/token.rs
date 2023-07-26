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
    Function,
    Let,
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            // Keywords
            "fn" => Self::Function,
            "let" => Self::Let,
            _ => {
                if value.chars().all(|b| b.is_ascii_digit()) {
                    Self::Int(value)
                } else {
                    Self::Ident(value)
                }
            }
        }
    }
}
