#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
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

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    r#type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(r#type: TokenType, literal: String) -> Token {
        Token { r#type, literal }
    }
}
