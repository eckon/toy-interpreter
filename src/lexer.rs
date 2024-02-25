use crate::token::{Token, TokenType};

#[derive(Default, Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
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

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        // whitespaces have no meaning for now - done here to keep not concat variable names etc.
        while let Some(ch) = self.ch {
            if !matches!(ch, '\n' | '\r' | '\t' | ' ') {
                break;
            }
            self.read_char();
        }

        let token = match self.ch {
            None => Token::new(TokenType::Eof, "".to_string()),
            Some(ch @ '=') => Token::new(TokenType::Assign, ch.to_string()),
            Some(ch @ ';') => Token::new(TokenType::Semicolon, ch.to_string()),
            Some(ch @ '(') => Token::new(TokenType::Lparen, ch.to_string()),
            Some(ch @ ')') => Token::new(TokenType::Rparen, ch.to_string()),
            Some(ch @ ',') => Token::new(TokenType::Comma, ch.to_string()),
            Some(ch @ '+') => Token::new(TokenType::Plus, ch.to_string()),
            Some(ch @ '{') => Token::new(TokenType::Lbrace, ch.to_string()),
            Some(ch @ '}') => Token::new(TokenType::Rbrace, ch.to_string()),
            Some('0'..='9') => {
                let start_position = self.position;
                while let Some(ch) = self.ch {
                    if !ch.is_ascii_digit() {
                        break;
                    }
                    self.read_char();
                }

                let literal = self.input[start_position..self.position].to_string();

                // need to return to not skip the next token
                return Token::new(TokenType::Int, literal);
            }
            Some('a'..='z' | 'A'..='Z' | '_') => {
                // read identifier, allow snake_case
                let start_position = self.position;
                while let Some(ch) = self.ch {
                    if !matches!(ch, 'a'..='z' | 'A'..='Z' | '_') {
                        break;
                    }
                    self.read_char();
                }

                let literal = self.input[start_position..self.position].to_string();
                let ident_type = match literal.as_str() {
                    "fn" => TokenType::Function,
                    "let" => TokenType::Let,
                    _ => TokenType::Ident,
                };

                // need to return to not skip the next token
                return Token::new(ident_type, literal);
            }
            Some(ch) => Token::new(TokenType::Illegal, ch.to_string()),
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        token::{Token, TokenType},
    };

    #[test]
    fn test_simple_input() {
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
            assert_eq!(token, test)
        }
    }

    #[test]
    fn test_ignore_eof_as_number() {
        let input = "3210";
        let tests = [
            Token::new(TokenType::Int, "3210".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];

        let mut l = Lexer::new(input.to_string());

        for test in tests {
            let token = l.next_token();
            assert_eq!(token, test)
        }
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let tests = [
            Token::new(TokenType::Eof, "".to_string()),
        ];

        let mut l = Lexer::new(input.to_string());

        for test in tests {
            let token = l.next_token();
            assert_eq!(token, test)
        }
    }

    #[test]
    fn test_complex_input() {
        let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            ";

        let tests = [
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "five".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "ten".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "add".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Function, "fn".to_string()),
            Token::new(TokenType::Lparen, "(".to_string()),
            Token::new(TokenType::Ident, "x".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Ident, "y".to_string()),
            Token::new(TokenType::Rparen, ")".to_string()),
            Token::new(TokenType::Lbrace, "{".to_string()),
            Token::new(TokenType::Ident, "x".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::Ident, "y".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Rbrace, "}".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "result".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Ident, "add".to_string()),
            Token::new(TokenType::Lparen, "(".to_string()),
            Token::new(TokenType::Ident, "five".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Ident, "ten".to_string()),
            Token::new(TokenType::Rparen, ")".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];

        let mut l = Lexer::new(input.to_string());

        for test in tests {
            let token = l.next_token();
            assert_eq!(token, test, "\nNEXT POSITION:\n{}", {
                let mut context = l.input.to_string();
                let current_token = context.get(l.position..l.position + 1).unwrap();
                context.replace_range(
                    l.position..l.position + 1,
                    format!(">{}<", current_token).as_str(),
                );
                context
            });
        }
    }
}
