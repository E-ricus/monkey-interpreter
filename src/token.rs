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
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreatThan,
    Eq,
    NotEq,
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
    True,
    False,
    Else,
    If,
    Return,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '=' => Self::Assign,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '!' => Self::Bang,
            '/' => Self::Slash,
            '*' => Self::Asterisk,
            ';' => Self::Semicolon,
            '(' => Self::Lparen,
            ')' => Self::Rparen,
            ',' => Self::Comma,
            '{' => Self::Lbrace,
            '}' => Self::Rbrace,
            '<' => Self::LessThan,
            '>' => Self::GreatThan,
            '\0' => Self::Eof,
            _ => Self::Illegal,
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            // Keywords
            "fn" => Self::Function,
            "let" => Self::Let,
            "true" => Self::True,
            "false" => Self::False,
            "if" => Self::If,
            "else" => Self::Else,
            "return" => Self::Return,
            "==" => Self::Eq,
            "!=" => Self::NotEq,
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
