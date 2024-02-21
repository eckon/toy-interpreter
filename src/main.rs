#[derive(Debug, Eq, PartialEq)]
#[allow(dead_code)]
enum TokenType {
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

#[derive(Debug)]
#[allow(dead_code)]
struct Token {
    r#type: TokenType,
    literal: String,
}

#[allow(dead_code)]
impl Token {
    fn new(r#type: TokenType, literal: String) -> Token {
        Token { r#type, literal }
    }
}

#[derive(Default, Debug)]
#[allow(dead_code)]
struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: char,
}

#[allow(dead_code)]
impl Lexer {
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            ..Default::default()
        };

        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len().try_into().unwrap() {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().nth(self.read_position as usize).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
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

fn main() {
    println!("Hello, interpreter!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = [
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::Lparen, "(".to_string()),
            Token::new(TokenType::Rparen, ")".to_string()),
            Token::new(TokenType::Lbrace, "{".to_string()),
            Token::new(TokenType::Rbrace, "}".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];

        let mut l = Lexer::new(input.to_string());

        for test in tests {
            let token = l.next_token();
            assert_eq!(token.r#type, test.r#type);
            assert_eq!(token.literal, test.literal);
        }
    }
}
