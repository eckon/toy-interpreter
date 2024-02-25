#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,
    Float,

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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    r#type: TokenType,
    literal: Option<String>,
}

impl Token {
    pub fn new(r#type: TokenType, literal: Option<String>) -> Token {
        Token { r#type, literal }
    }

    pub fn get_type(&self) -> TokenType {
        self.r#type.clone()
    }

    pub fn get_literal(&self) -> Option<String> {
        self.literal.clone()
    }
}
