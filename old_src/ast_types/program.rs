use super::{BlockStatement, StatementEnum};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub code: BlockStatement
}

impl Program {
    pub fn new() -> Self {
        Self {
            code: BlockStatement::default()
        }
    }

    pub fn add_statement(&mut self, global_statement: StatementEnum) {
        self.code.body.push(global_statement); // statements added directly to the program are global statements
    }
}