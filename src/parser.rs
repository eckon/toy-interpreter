use crate::{ast::Program, lexer::Lexer, token::Token};

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

    pub fn parse_program(self) -> Program {
        Program::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::Parser;

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

        println!("{:?}", program);
    }
}
