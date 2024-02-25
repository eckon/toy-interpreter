mod lexer;
mod token;

fn main() {
    println!("Hello, interpreter!");
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        token::{Token, TokenType},
    };

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
            assert_eq!(token, test);
        }
    }
}
