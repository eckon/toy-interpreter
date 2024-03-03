use crate::token::Token;

#[derive(Debug)]
pub enum Statement {
    Let {
        token: Token,
        name: Identifier,
        value: Node,
    },
}

#[derive(Debug)]
pub struct Identifier {
    token: Token,
    value: String,
}

#[derive(Debug)]
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

    pub fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}
