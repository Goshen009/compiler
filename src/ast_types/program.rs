









use super::StatementEnum;


#[derive(Debug, PartialEq)]
pub struct Program {
    pub code: Vec<StatementEnum>
}

impl Program {
    pub fn new() -> Self {
        Self {
            code: Vec::new()
        }
    }

    pub fn add_statement(&mut self, global_statement: StatementEnum) {
        // statements added directly to the program are global statements
        self.code.push(global_statement);
    }
}