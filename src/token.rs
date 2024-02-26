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
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    NotEq,

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
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    r#type: TokenType,
    literal: String,
    start_postion: usize,
}

impl Token {
    pub fn new(r#type: TokenType, literal: String, start_postion: usize) -> Token {
        Token {
            r#type,
            literal,
            start_postion,
        }
    }

    pub fn get_type(&self) -> TokenType {
        self.r#type.clone()
    }

    pub fn get_literal(&self) -> String {
        self.literal.clone()
    }

    pub fn get_position(&self) -> (usize, usize) {
        (
            self.start_postion,
            self.start_postion + self.get_literal().len(),
        )
    }
}
