#[derive(Debug)]
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
struct Token {
    r#type: TokenType,
    literal: String,
}

impl Token {
    fn new(r#type: TokenType, literal: String) -> Token {
        Token { r#type, literal }
    }
}

fn main() {
    let x = Token::new(TokenType::Assign, "=".to_string());
    print!("{:?}", x);

    println!("Hello, interpreter!");
}

#[cfg(test)]
mod tests {
    use crate::Token;
    use crate::TokenType;

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

        // create a lexer
        // iterate over the tests
        // assert_eq!(2 + 2, 4);
    }
}
