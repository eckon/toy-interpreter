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
    length: usize,
}

impl Token {
    pub fn new(r#type: TokenType, start_postion: usize) -> Token {
        let length = match r#type {
            TokenType::Assign
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Bang
            | TokenType::Asterisk
            | TokenType::Slash
            | TokenType::Lt
            | TokenType::Gt
            | TokenType::Comma
            | TokenType::Semicolon
            | TokenType::Lparen
            | TokenType::Rparen
            | TokenType::Lbrace
            | TokenType::Rbrace => 1,
            TokenType::Illegal | TokenType::Eof => 0,
            _ => 0,
        };

        Token {
            r#type,
            literal: None,
            start_postion,
            length,
        }
    }

    pub fn set_length(&mut self, length: usize) {
        self.length = length;
    }

    pub fn set_literal(&mut self, literal: String) {
        self.literal = Some(literal);
    }

    pub fn get_type(&self) -> TokenType {
        self.r#type.clone()
    }

    pub fn get_literal(&self) -> Option<String> {
        self.literal.clone()
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.start_postion, self.start_postion + self.length)
    }
}
