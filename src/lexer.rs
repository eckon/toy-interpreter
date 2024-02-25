use crate::token::{Token, TokenType};

#[derive(Default, Debug)]
pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            ..Default::default()
        };

        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len().try_into().unwrap() {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().nth(self.read_position as usize).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let literal = self.ch.to_string();
        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, literal),
            ';' => Token::new(TokenType::Semicolon, literal),
            '(' => Token::new(TokenType::Lparen, literal),
            ')' => Token::new(TokenType::Rparen, literal),
            ',' => Token::new(TokenType::Comma, literal),
            '+' => Token::new(TokenType::Plus, literal),
            '{' => Token::new(TokenType::Lbrace, literal),
            '}' => Token::new(TokenType::Rbrace, literal),
            '0' => Token::new(TokenType::Eof, "".to_string()),
            _ => Token::new(TokenType::Illegal, "".to_string()),
        };

        self.read_char();
        token
    }
}
