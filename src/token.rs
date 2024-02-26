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
    literal: Option<String>,
    start_postion: usize,
}

impl Token {
    pub fn new(r#type: TokenType, start_postion: usize) -> Token {
        Token {
            r#type,
            literal: None,
            start_postion,
        }
    }

    pub fn set_literal(&mut self, literal: String) {
        self.literal = Some(literal);
    }

    pub fn get_length(&self) -> usize {
        match self.r#type {
            TokenType::Illegal | TokenType::Eof => return 0,
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
            | TokenType::Rbrace => return 1,
            TokenType::Eq | TokenType::NotEq | TokenType::Function | TokenType::If => return 2,
            TokenType::Let => return 3,
            TokenType::True | TokenType::Else => return 4,
            TokenType::False | TokenType::Return => return 5,
            _ => {}
        };

        if let Some(literal) = &self.literal {
            return literal.len();
        }

        panic!("Token has no length or literal -- {:?}", self);
    }

    pub fn get_type(&self) -> TokenType {
        self.r#type.clone()
    }

    pub fn get_literal(&self) -> Option<String> {
        self.literal.clone()
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.start_postion, self.start_postion + self.get_length())
    }
}
