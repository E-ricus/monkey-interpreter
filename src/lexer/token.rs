use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // TODO: Maybe ilegal is enough
    Empty,
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Empty => write!(f, ""),
            Token::Illegal => write!(f, ""),
            Token::Eof => write!(f, "\0"),
            Token::Ident(value) | Token::Int(value) => write!(f, "{value}"),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::LessThan => write!(f, "<"),
            Token::GreatThan => write!(f, ">"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbrace => write!(f, "{{"),
            Token::Rbrace => write!(f, "}}"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
        }
    }
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
