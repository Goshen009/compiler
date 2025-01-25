use std::fmt::Debug;
use super::Token;

pub type ExprGeneral = Box<dyn ExpressionTrait>;
pub type StmtGeneral = Box<dyn StatementTrait>;

pub trait StatementTrait: Debug { }
impl StatementTrait for BlockStatement { }
impl StatementTrait for ExpressionStatement { }

impl Debug for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("BlockStatment\n");
        for stmt in self.body.iter() {
            let val = format!("{:#?}", stmt);
            res.push_str(&val);
        }
        write!(f, "{res}")
    }
}

impl Debug for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.expression)   
    }
}

pub struct BlockStatement {
    pub body : Vec<StmtGeneral>,
}
pub struct ExpressionStatement {
    pub expression: ExprGeneral,
}


pub trait ExpressionTrait: Debug { }
impl ExpressionTrait for NumberExpression { }
impl ExpressionTrait for StringExpression { }
impl ExpressionTrait for SymbolExpression { }
impl ExpressionTrait for BinaryExpression { }

#[derive(Debug)]
pub struct NumberExpression {
    pub value: i32,
}

#[derive(Debug)]
pub struct StringExpression {
    pub value: String,
}

#[derive(Debug)]
pub struct SymbolExpression {
    pub value: String,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: ExprGeneral,
    pub operator: Token,
    pub right: ExprGeneral
}