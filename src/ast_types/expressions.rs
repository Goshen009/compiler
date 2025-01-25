use std::collections::HashMap;
use std::fmt::Debug;
use super::Token;

#[derive(Debug, PartialEq)]
pub enum ExpressionEnum {
    EmptyTuple,
    Number(NumberExpression),
    String(StringExpression),
    Symbol(SymbolExpression),
    Array(Box<ArrayExpression>),
    Binary(Box<BinaryExpression>),
    Prefix(Box<PrefixExpression>),
    Assignment(Box<AssignmentExpression>),
    TupleAssignment(Box<TupleAssignmentExpression>),
    StructAssignment(Box<StructAssignmentExpression>),
    FunctionCall(Box<FunctionCallExpression>), // this will work out as an expression statement(). in the place where it get's the stmt_type. do it such that if it's a symbol it checks the next token to see if it's an open brackets. if it is return a null.
}

#[derive(Debug, PartialEq)]
pub struct ArrayExpression {
    pub array: Vec<ExpressionEnum>
}

#[derive(Debug, PartialEq)]
pub struct NumberExpression {
    pub value: i32,
}

#[derive(Debug, PartialEq)]
pub struct StringExpression {
    pub value: String
}

#[derive(Debug, PartialEq)]
pub struct SymbolExpression {
    pub value: String
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    pub left: ExpressionEnum,
    pub operator: Token,
    pub right: ExpressionEnum
}

#[derive(Debug, PartialEq)]
pub struct PrefixExpression {
    pub sign: Token,
    pub right_hand: ExpressionEnum
}

#[derive(Debug, PartialEq)]
pub struct AssignmentExpression {
    pub left: ExpressionEnum,
    pub operator_kind: Token,
    pub right: ExpressionEnum,
}

#[derive(Debug, PartialEq)]
pub struct StructAssignmentExpression {
    pub struct_name: String,
    pub struct_fields: HashMap<String, ExpressionEnum>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCallExpression {
    pub function_name: String,
    pub function_argument: ExpressionEnum // since a tuple with a single value is evaluated to a normal expression, this should take the ExpressionEnum and not Vec<ExpressionEnum>
}

#[derive(Debug, PartialEq)]
pub struct TupleAssignmentExpression {
    pub tuple: Vec<ExpressionEnum>,
}