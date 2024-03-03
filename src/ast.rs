use crate::token::Token;

#[derive(Debug)]
pub enum Statement {
    Let {
        token: Token,
        name: Identifier,
        // value: Node,
    },
    Return {
        token: Token,
        // value: Node,
    },
}

#[derive(Debug, Default)]
pub struct Identifier {
    token: Token,
    value: String,
}

#[derive(Debug, Default)]
pub struct Node {
    token: Token,
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Default::default(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    pub fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Identifier {
        Identifier { token, value }
    }

    pub fn get_token(&self) -> &Token {
        &self.token
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}
