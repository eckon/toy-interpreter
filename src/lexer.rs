use crate::token::{Token, TokenType};

#[derive(Default, Debug, Clone)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            ..Default::default()
        };

        lexer.read_char();
        lexer
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

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.read_position)
    }

    pub fn next_token(&mut self) -> Token {
        // whitespaces have no meaning for now - done here to keep not concat variable names etc.
        while let Some(ch) = self.ch {
            if !matches!(ch, '\n' | '\r' | '\t' | ' ') {
                break;
            }
            self.read_char();
        }

        // store start postition even after reading more characters
        let start_pos = self.position;
        let token = match self.ch {
            None => Token::new(TokenType::Eof, "".into(), start_pos),
            Some(';') => Token::new(TokenType::Semicolon, ";".into(), start_pos),
            Some('(') => Token::new(TokenType::Lparen, "(".into(), start_pos),
            Some(')') => Token::new(TokenType::Rparen, ")".into(), start_pos),
            Some('{') => Token::new(TokenType::Lbrace, "{".into(), start_pos),
            Some('}') => Token::new(TokenType::Rbrace, "}".into(), start_pos),
            Some(',') => Token::new(TokenType::Comma, ",".into(), start_pos),
            Some('+') => Token::new(TokenType::Plus, "+".into(), start_pos),
            Some('-') => Token::new(TokenType::Minus, "-".into(), start_pos),
            Some('*') => Token::new(TokenType::Asterisk, "*".into(), start_pos),
            Some('/') => Token::new(TokenType::Slash, "/".into(), start_pos),
            Some('<') => Token::new(TokenType::Lt, "<".into(), start_pos),
            Some('>') => Token::new(TokenType::Gt, ">".into(), start_pos),
            Some('=') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::new(TokenType::Eq, "==".into(), start_pos)
                } else {
                    Token::new(TokenType::Assign, "=".into(), start_pos)
                }
            }
            Some('!') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=".into(), start_pos)
                } else {
                    Token::new(TokenType::Bang, "!".into(), start_pos)
                }
            }

            Some('0'..='9') => {
                enum NumberType {
                    Int,
                    Float,
                }

                let mut number_type = NumberType::Int;

                let start_pos = self.position;
                while let Some(ch) = self.ch {
                    if ch == '.' {
                        number_type = NumberType::Float;
                    }

                    if !ch.is_ascii_digit() && ch != '.' {
                        break;
                    }

                    self.read_char();
                }

                let literal = self.input[start_pos..self.position].to_string();
                let token = match number_type {
                    NumberType::Int => Token::new(
                        TokenType::Int,
                        // remove leading 0s
                        literal.trim_start_matches('0').to_string(),
                        start_pos,
                    ),
                    NumberType::Float => Token::new(
                        TokenType::Float,
                        // remove trailing 0s and 0s after 0. and 0.0
                        literal
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string(),
                        start_pos,
                    ),
                };

                // need to return to not skip the next token
                return token;
            }
            Some('a'..='z' | 'A'..='Z' | '_') => {
                // read identifier, allow snake_case and numbers (not at start)
                let start_pos = self.position;
                while let Some(ch) = self.ch {
                    if !matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                        break;
                    }
                    self.read_char();
                }

                let identifier = self.input[start_pos..self.position].to_string();
                let token = match identifier.as_str() {
                    "fn" => Token::new(TokenType::Function, "fn".into(), start_pos),
                    "let" => Token::new(TokenType::Let, "let".into(), start_pos),
                    "true" => Token::new(TokenType::True, "true".into(), start_pos),
                    "false" => Token::new(TokenType::False, "false".into(), start_pos),
                    "if" => Token::new(TokenType::If, "if".into(), start_pos),
                    "else" => Token::new(TokenType::Else, "else".into(), start_pos),
                    "return" => Token::new(TokenType::Return, "return".into(), start_pos),
                    _ => Token::new(TokenType::Ident, identifier, start_pos),
                };

                // need to return to not skip the next token
                return token;
            }
            Some(ch) => Token::new(TokenType::Illegal, ch.to_string(), start_pos),
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

            if (true)
        ";

        let mut l = Lexer::new(input.to_string());
        loop {
            let token = l.next_token();
            if token.get_type() == TokenType::Eof {
                break;
            }

            if token.get_literal() == "this_is_wanted" {
                assert_eq!(
                    token.get_position(),
                    (114, 128),
                    "{}",
                    context_formatting(input.to_string(), &token)
                )
            }

            if token.get_type() == TokenType::True {
                assert_eq!(
                    token.get_position(),
                    (226, 230),
                    "{}",
                    context_formatting(input.to_string(), &token)
                )
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
            (TokenType::Let, "let"),
            (TokenType::Ident, "an0th3e_5"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
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
        let tests = vec![(TokenType::Int, "1230"), (TokenType::Eof, "")];

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
            (TokenType::Let, "let"),
            (TokenType::Ident, "x"),
            (TokenType::Assign, "="),
            (TokenType::Float, "3.14"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "y"),
            (TokenType::Assign, "="),
            (TokenType::Float, "2"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "z"),
            (TokenType::Assign, "="),
            (TokenType::Float, "0.5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
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
        let tests = vec![(TokenType::Int, "3210"), (TokenType::Eof, "")];

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
        let test = (TokenType::Eof, "");

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
    fn test_token_meaning_complex() {
        let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);

            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            5 == 5;
            5 != 10;
            ";

        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::Rparen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::Lparen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::Lbrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Int, "5"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::NotEq, "!="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
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
