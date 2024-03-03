use std::fmt;

use crate::{
    ast::{Identifier, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct ParserError {
    message: String,
    token: Token,
}

#[derive(Debug)]
pub struct ParserErrorList {
    errors: Vec<ParserError>,
}

impl ParserErrorList {
    pub fn new() -> ParserErrorList {
        ParserErrorList {
            errors: Vec::<ParserError>::new(),
        }
    }

    pub fn add_error(&mut self, error: ParserError) {
        self.errors.push(error);
    }

    pub fn get_errors(&self) -> &Vec<ParserError> {
        &self.errors
    }
}

impl fmt::Display for ParserErrorList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for e in self.get_errors() {
            writeln!(f, "msg: {}, token: {:?}", e.message, e.token)?
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            ..Default::default()
        };

        // fill the current and next token with real values
        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn expect_next_token(&mut self, expected_type: TokenType) -> Result<(), ParserError> {
        if self.next_token.get_type() == expected_type {
            self.next_token();
            Ok(())
        } else {
            Err(ParserError {
                message: format!(
                    "Expected next token to be of type \"{:?}\", but got \"{:?}\"",
                    expected_type,
                    self.next_token.get_type()
                ),
                token: self.next_token.clone(),
            })
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token.get_type() {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            n => Err(ParserError {
                message: format!("Unknown token type for statement: {:?}", n),
                token: self.current_token.clone(),
            }),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let token = self.current_token.clone();

        self.expect_next_token(TokenType::Ident)?;

        let identifier = Identifier::new(
            self.current_token.clone(),
            self.current_token.get_literal().clone(),
        );

        self.expect_next_token(TokenType::Assign)?;

        // for now, just skip the expression
        loop {
            self.next_token();
            if self.current_token.get_type() == TokenType::Semicolon {
                break;
            }
        }

        Ok(Statement::Let {
            token,
            name: identifier,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        let token = self.current_token.clone();

        // for now, just skip the expression
        loop {
            self.next_token();
            if self.current_token.get_type() == TokenType::Semicolon {
                break;
            }
        }

        Ok(Statement::Return { token })
    }

    pub fn parse_program(mut self) -> Result<Program, ParserErrorList> {
        let mut program = Program::new();
        let mut errors = ParserErrorList::new();

        loop {
            let statement = self.parse_statement();
            match statement {
                Ok(statement) => program.add_statement(statement),
                Err(e) => errors.add_error(e),
            }

            self.next_token();
            if self.current_token.get_type() == TokenType::Eof {
                break;
            }
        }

        if errors.get_errors().is_empty() {
            Ok(program)
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{ast::Statement, lexer::Lexer};

    #[test]
    fn test_return_statement() {
        let input = "
            return \"foo\";
            return 5;
            return add(1, 3);
        ";

        let lexer = Lexer::new(input.into());
        let parser = Parser::new(lexer);
        let program = parser.parse_program();

        if let Err(errors) = program {
            println!("{}", errors);
            panic!("Program could not be parsed correctly");
        }

        let program = program.unwrap();

        assert_eq!(program.get_statements().len(), 3);

        for s in program.get_statements() {
            assert!(matches!(s, Statement::Return { .. }));
        }
    }

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 26957834;
        ";

        let lexer = Lexer::new(input.into());
        let parser = Parser::new(lexer);
        let program = parser.parse_program();

        if let Err(errors) = program {
            println!("{}", errors);
            panic!("Program could not be parsed correctly");
        }

        let program = program.unwrap();

        assert_eq!(program.get_statements().len(), 3);

        let expected_identifier = ["x", "y", "foobar"];

        for (i, s) in program.get_statements().iter().enumerate() {
            assert!(matches!(s, Statement::Let { .. }));

            if let Statement::Let { name, .. } = s {
                assert_eq!(expected_identifier[i], name.get_value());
            }
        }
    }
}
