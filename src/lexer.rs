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
            None => Token::new(TokenType::Eof, None),
            Some('=') => Token::new(TokenType::Assign, None),
            Some(';') => Token::new(TokenType::Semicolon, None),
            Some('(') => Token::new(TokenType::Lparen, None),
            Some(')') => Token::new(TokenType::Rparen, None),
            Some(',') => Token::new(TokenType::Comma, None),
            Some('+') => Token::new(TokenType::Plus, None),
            Some('{') => Token::new(TokenType::Lbrace, None),
            Some('}') => Token::new(TokenType::Rbrace, None),
            Some('0'..='9') => {
                enum NumberType {
                    Int,
                    Float,
                }

                let mut number_type = NumberType::Int;

                let start_position = self.position;
                while let Some(ch) = self.ch {
                    if ch == '.' {
                        number_type = NumberType::Float;
                    }

                    if !ch.is_ascii_digit() && ch != '.' {
                        break;
                    }

                    self.read_char();
                }

                let literal = self.input[start_position..self.position].to_string();
                let token = match number_type {
                    NumberType::Int => Token::new(
                        TokenType::Int,
                        // remove leading 0s
                        Some(literal.trim_start_matches('0').to_string()),
                    ),
                    NumberType::Float => Token::new(
                        TokenType::Float,
                        Some(
                            // remove trailing 0s and 0s after 0. and 0.0
                            literal
                                .trim_end_matches('0')
                                .trim_end_matches('.')
                                .to_string(),
                        ),
                    ),
                };

                // need to return to not skip the next token
                return token;
            }
            Some('a'..='z' | 'A'..='Z' | '_') => {
                // read identifier, allow snake_case and numbers (not at start)
                let start_position = self.position;
                while let Some(ch) = self.ch {
                    if !matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                        break;
                    }
                    self.read_char();
                }

                let identifier = self.input[start_position..self.position].to_string();
                let token = match identifier.as_str() {
                    "fn" => Token::new(TokenType::Function, None),
                    "let" => Token::new(TokenType::Let, None),
                    _ => Token::new(TokenType::Ident, Some(identifier)),
                };

                // need to return to not skip the next token
                return token;
            }
            Some(ch) => Token::new(TokenType::Illegal, Some(ch.to_string())),
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

    fn assert_token_tests(tests: Vec<Token>, input: &str) {
        let mut l = Lexer::new(input.to_string());

        for test in tests {
            let token = l.next_token();
            assert_eq!(token, test, "\nNEXT POSITION:{}", {
                let context = if l.input.len() < 10 {
                    " FIX logging, currently short input will panic".to_string()
                } else {
                    let mut context = l.input.to_string();
                    let current_token = context.get(l.position..l.position + 1).unwrap_or("");
                    context.replace_range(
                        l.position..l.position + 1,
                        format!("#{}#", current_token).as_str(),
                    );
                    format!("\n{}", context)
                };
                context
            });
        }
    }

    #[test]
    fn test_simple() {
        let input = "=+(){},;";
        let tests = vec![
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Plus, None),
            Token::new(TokenType::Lparen, None),
            Token::new(TokenType::Rparen, None),
            Token::new(TokenType::Lbrace, None),
            Token::new(TokenType::Rbrace, None),
            Token::new(TokenType::Comma, None),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Eof, None),
        ];

        assert_token_tests(tests, input);
    }

    #[test]
    fn test_variable_name_with_number() {
        let input = "let an0th3e_5 = 5;";
        let tests = vec![
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("an0th3e_5".to_string())),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Int, Some("5".to_string())),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Eof, None),
        ];

        assert_token_tests(tests, input);
    }

    #[test]
    fn test_integer_with_leading_zero() {
        let input = "001230";
        let tests = vec![
            Token::new(TokenType::Int, Some("1230".to_string())),
            Token::new(TokenType::Eof, None),
        ];

        assert_token_tests(tests, input);
    }

    #[test]
    fn test_float() {
        let input = "
            let x = 3.14;
            let y = 2.00;
            let z = 0.5;";
        let tests = vec![
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("x".to_string())),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Float, Some("3.14".to_string())),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("y".to_string())),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Float, Some("2".to_string())),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("z".to_string())),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Float, Some("0.5".to_string())),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Eof, None),
        ];

        assert_token_tests(tests, input);
    }

    #[test]
    fn test_ignore_eof_as_number() {
        let input = "3210";
        let tests = vec![
            Token::new(TokenType::Int, Some("3210".to_string())),
            Token::new(TokenType::Eof, None),
        ];

        assert_token_tests(tests, input);
    }

    #[test]
    fn test_empty() {
        let input = "";
        let tests = vec![Token::new(TokenType::Eof, None)];

        assert_token_tests(tests, input);
    }

    #[test]
    fn test_complex() {
        let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            ";

        let tests = vec![
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("five".to_string())),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Int, Some("5".to_string())),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("ten".to_string())),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Int, Some("10".to_string())),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("add".to_string())),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Function, None),
            Token::new(TokenType::Lparen, None),
            Token::new(TokenType::Ident, Some("x".to_string())),
            Token::new(TokenType::Comma, None),
            Token::new(TokenType::Ident, Some("y".to_string())),
            Token::new(TokenType::Rparen, None),
            Token::new(TokenType::Lbrace, None),
            Token::new(TokenType::Ident, Some("x".to_string())),
            Token::new(TokenType::Plus, None),
            Token::new(TokenType::Ident, Some("y".to_string())),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Rbrace, None),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("result".to_string())),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Ident, Some("add".to_string())),
            Token::new(TokenType::Lparen, None),
            Token::new(TokenType::Ident, Some("five".to_string())),
            Token::new(TokenType::Comma, None),
            Token::new(TokenType::Ident, Some("ten".to_string())),
            Token::new(TokenType::Rparen, None),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Eof, None),
        ];

        assert_token_tests(tests, input);
    }
}
