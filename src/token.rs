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
    start_postion: usize,
    end_position: usize,
}

impl Token {
    pub fn new(r#type: TokenType, literal: Option<String>, position: (usize, usize)) -> Token {
        Token {
            r#type,
            literal,
            start_postion: position.0,
            end_position: position.1,
        }
    }

    pub fn get_type(&self) -> TokenType {
        self.r#type.clone()
    }

    pub fn get_literal(&self) -> Option<String> {
        self.literal.clone()
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.start_postion, self.end_position)
    }
}
