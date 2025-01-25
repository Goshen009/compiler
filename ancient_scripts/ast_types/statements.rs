use std::fmt::Debug;
use super::{StatementTrait, StatementObj, ExpressionObj, TypeObj};

impl StatementTrait for BlockStatement { }
impl StatementTrait for ExpressionStatement { }
impl StatementTrait for VariableDeclarationStatement { }

#[derive(Debug)]
pub struct BlockStatement {
    pub body: Vec<StatementObj>
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: ExpressionObj
}

#[derive(Debug)]
pub struct VariableDeclarationStatement {
    pub variable_name: String,
    pub is_constant: bool,
    pub value: ExpressionObj,
    pub explicit_type: Option<TypeObj>
}