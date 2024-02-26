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

    fn highlighted_input(input: String, token: &Token) -> String {
        let mut context = input.to_string();
        context.insert(token.get_position().0, '>');
        context.insert(token.get_position().1 + 1, '<');
        context
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
            None => Token::new(TokenType::Eof, self.position),
            Some('=') => Token::new(TokenType::Assign, self.position),
            Some(';') => Token::new(TokenType::Semicolon, self.position),
            Some('(') => Token::new(TokenType::Lparen, self.position),
            Some(')') => Token::new(TokenType::Rparen, self.position),
            Some('{') => Token::new(TokenType::Lbrace, self.position),
            Some('}') => Token::new(TokenType::Rbrace, self.position),
            Some(',') => Token::new(TokenType::Comma, self.position),
            Some('+') => Token::new(TokenType::Plus, self.position),
            Some('-') => Token::new(TokenType::Minus, self.position),
            Some('!') => Token::new(TokenType::Bang, self.position),
            Some('*') => Token::new(TokenType::Asterisk, self.position),
            Some('/') => Token::new(TokenType::Slash, self.position),
            Some('<') => Token::new(TokenType::Lt, self.position),
            Some('>') => Token::new(TokenType::Gt, self.position),
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
                let mut token = match number_type {
                    NumberType::Int => {
                        let mut token = Token::new(TokenType::Int, start_position);
                        // remove leading 0s
                        token.set_literal(literal.trim_start_matches('0').to_string());
                        token
                    }
                    NumberType::Float => {
                        let mut token = Token::new(TokenType::Float, start_position);
                        token.set_literal(
                            // remove trailing 0s and 0s after 0. and 0.0
                            literal
                                .trim_end_matches('0')
                                .trim_end_matches('.')
                                .to_string(),
                        );
                        token
                    }
                };

                token.set_length(self.position - start_position);

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
                let mut token = match identifier.as_str() {
                    "fn" => Token::new(TokenType::Function, start_position),
                    "let" => Token::new(TokenType::Let, start_position),
                    _ => {
                        let mut token = Token::new(TokenType::Ident, start_position);
                        token.set_literal(identifier);
                        token
                    }
                };

                token.set_length(self.position - start_position);

                // need to return to not skip the next token
                return token;
            }
            Some(ch) => {
                let mut token = Token::new(TokenType::Illegal, self.position);
                token.set_literal(ch.to_string());
                token
            }
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

    fn context_formatting(input: String, token: &Token) -> String {
        format!(
            "\n\tTYPE: {:?}\n\tLITERAL: {:?}\n\tPOSITION: {:?}\n\tCONTEXT:\n{}\n",
            token.get_type(),
            token.get_literal(),
            token.get_position(),
            Lexer::highlighted_input(input, token)
        )
    }

    #[test]
    fn test_token_position_complex() {
        let input = "
            let x = 123;
            let add = fn(x, y) {
                x + y;
            };

            let this_is_wanted = 3.14;
            let add2 = fn(x, y) {
                x + y;
            };
        ";

        let mut l = Lexer::new(input.to_string());
        loop {
            let token = l.next_token();
            if token.get_type() == TokenType::Eof {
                break;
            }

            if let Some(t) = token.get_literal() {
                if t == "this_is_wanted" {
                    assert_eq!(
                        token.get_position(),
                        (114, 128),
                        "{}",
                        context_formatting(input.to_string(), &token)
                    )
                }
            }
        }
    }

    #[test]
    fn test_token_position_simple() {
        let input = "(){}";

        let mut l = Lexer::new(input.to_string());
        loop {
            let token = l.next_token();
            if token.get_type() == TokenType::Eof {
                break;
            }

            if token.get_type() == TokenType::Rparen {
                assert_eq!(
                    token.get_position(),
                    (1, 2),
                    "{}",
                    context_formatting(input.to_string(), &token)
                )
            }
        }
    }

    #[test]
    fn test_token_meaning_simple() {
        let input = "=+-<>!*/(){},;";
        let tests = vec![
            TokenType::Assign,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Lt,
            TokenType::Gt,
            TokenType::Bang,
            TokenType::Asterisk,
            TokenType::Slash,
            TokenType::Lparen,
            TokenType::Rparen,
            TokenType::Lbrace,
            TokenType::Rbrace,
            TokenType::Comma,
            TokenType::Semicolon,
            TokenType::Eof,
        ];

        let mut l = Lexer::new(input.to_string());
        for test in tests {
            let token = l.next_token();
            assert_eq!(
                token.get_type(),
                test,
                "{}",
                context_formatting(input.to_string(), &token)
            );
        }
    }

    #[test]
    fn test_variable_name_with_number() {
        let input = "let an0th3e_5 = 5;";
        let tests = vec![
            (TokenType::Let, None),
            (TokenType::Ident, Some("an0th3e_5".to_string())),
            (TokenType::Assign, None),
            (TokenType::Int, Some("5".to_string())),
            (TokenType::Semicolon, None),
            (TokenType::Eof, None),
        ];

        let mut l = Lexer::new(input.to_string());
        for test in tests {
            let token = l.next_token();
            assert_eq!(
                token.get_type(),
                test.0,
                "{}",
                context_formatting(input.to_string(), &token)
            );

            assert_eq!(
                token.get_literal(),
                test.1,
                "{}",
                context_formatting(input.to_string(), &token)
            );
        }
    }

    #[test]
    fn test_integer_with_leading_zero() {
        let input = "001230";
        let tests = vec![
            (TokenType::Int, Some("1230".to_string())),
            (TokenType::Eof, None),
        ];

        let mut l = Lexer::new(input.to_string());
        for test in tests {
            let token = l.next_token();
            assert_eq!(
                token.get_literal(),
                test.1,
                "{}",
                context_formatting(input.to_string(), &token)
            );
        }
    }

    #[test]
    fn test_float() {
        let input = "
            let x = 3.14;
            let y = 2.00;
            let z = 0.5;
        ";

        let tests = vec![
            (TokenType::Let, None),
            (TokenType::Ident, Some("x".to_string())),
            (TokenType::Assign, None),
            (TokenType::Float, Some("3.14".to_string())),
            (TokenType::Semicolon, None),
            (TokenType::Let, None),
            (TokenType::Ident, Some("y".to_string())),
            (TokenType::Assign, None),
            (TokenType::Float, Some("2".to_string())),
            (TokenType::Semicolon, None),
            (TokenType::Let, None),
            (TokenType::Ident, Some("z".to_string())),
            (TokenType::Assign, None),
            (TokenType::Float, Some("0.5".to_string())),
            (TokenType::Semicolon, None),
            (TokenType::Eof, None),
        ];

        let mut l = Lexer::new(input.to_string());
        for test in tests {
            let token = l.next_token();
            assert_eq!(
                token.get_type(),
                test.0,
                "{}",
                context_formatting(input.to_string(), &token)
            );

            assert_eq!(
                token.get_literal(),
                test.1,
                "{}",
                context_formatting(input.to_string(), &token)
            );
        }
    }

    #[test]
    fn test_ignore_eof_as_number() {
        let input = "3210";
        let tests = vec![
            (TokenType::Int, Some("3210".to_string())),
            (TokenType::Eof, None),
        ];

        let mut l = Lexer::new(input.to_string());
        for test in tests {
            let token = l.next_token();
            assert_eq!(
                token.get_type(),
                test.0,
                "{}",
                context_formatting(input.to_string(), &token)
            );

            assert_eq!(
                token.get_literal(),
                test.1,
                "{}",
                context_formatting(input.to_string(), &token)
            );
        }
    }

    #[test]
    fn test_empty() {
        let input = "";
        let test = (TokenType::Eof, None);

        let mut l = Lexer::new(input.to_string());
        let token = l.next_token();

        assert_eq!(
            token.get_type(),
            test.0,
            "{}",
            context_formatting(input.to_string(), &token)
        );

        assert_eq!(
            token.get_literal(),
            test.1,
            "{}",
            context_formatting(input.to_string(), &token)
        );
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
            (TokenType::Let, None),
            (TokenType::Ident, Some("five".to_string())),
            (TokenType::Assign, None),
            (TokenType::Int, Some("5".to_string())),
            (TokenType::Semicolon, None),
            (TokenType::Let, None),
            (TokenType::Ident, Some("ten".to_string())),
            (TokenType::Assign, None),
            (TokenType::Int, Some("10".to_string())),
            (TokenType::Semicolon, None),
            (TokenType::Let, None),
            (TokenType::Ident, Some("add".to_string())),
            (TokenType::Assign, None),
            (TokenType::Function, None),
            (TokenType::Lparen, None),
            (TokenType::Ident, Some("x".to_string())),
            (TokenType::Comma, None),
            (TokenType::Ident, Some("y".to_string())),
            (TokenType::Rparen, None),
            (TokenType::Lbrace, None),
            (TokenType::Ident, Some("x".to_string())),
            (TokenType::Plus, None),
            (TokenType::Ident, Some("y".to_string())),
            (TokenType::Semicolon, None),
            (TokenType::Rbrace, None),
            (TokenType::Semicolon, None),
            (TokenType::Let, None),
            (TokenType::Ident, Some("result".to_string())),
            (TokenType::Assign, None),
            (TokenType::Ident, Some("add".to_string())),
            (TokenType::Lparen, None),
            (TokenType::Ident, Some("five".to_string())),
            (TokenType::Comma, None),
            (TokenType::Ident, Some("ten".to_string())),
            (TokenType::Rparen, None),
            (TokenType::Semicolon, None),
            (TokenType::Eof, None),
        ];

        let mut l = Lexer::new(input.to_string());
        for test in tests {
            let token = l.next_token();
            assert_eq!(
                token.get_type(),
                test.0,
                "{}",
                context_formatting(input.to_string(), &token)
            );

            assert_eq!(
                token.get_literal(),
                test.1,
                "{}",
                context_formatting(input.to_string(), &token)
            );
        }
    }
}
