use std::{collections::HashMap, fmt::Debug};
use super::{ExpressionEnum, TypesEnum};

#[derive(Debug, PartialEq)]
pub enum StatementEnum {
    // Block(Box<BlockStatement>),
    // Expression(Box<ExpressionStatement>),
    VariableDeclaration(Box<VariableDeclarationStatement>),
    TupleDeclaration(Box<TupleDeclarationStatement>),
    StructDeclaration(Box<StructDeclarationStatement>),
    FunctionDeclaration(Box<FunctionDeclarationStatement>),
}

#[derive(Debug, PartialEq, Default)]
pub struct BlockStatement {
    pub body: Vec<StatementEnum>,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: ExpressionEnum
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclarationStatement {
    pub variable_name: String,
    pub is_constant: bool,
    pub value: ExpressionEnum,
    pub explicit_type: TypesEnum
}

#[derive(Debug, PartialEq)]
pub struct TupleDeclarationStatement {
    pub is_constant: bool,
    pub variable_name_and_type: Vec<(String, TypesEnum)>,
    pub value: ExpressionEnum,
}

#[derive(Debug, PartialEq)]
pub struct StructDeclarationStatement {
    pub struct_name: String,
    pub struct_fields: HashMap<String, TypesEnum>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclarationStatement {
    pub function_name: String,
    pub function_parameters: Vec<(String, TypesEnum)>, // needs to be vector because of the ordering.
    pub function_return_type: TypesEnum, // function may or may not have return type
    pub function_body: BlockStatement,
}