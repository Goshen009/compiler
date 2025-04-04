use std::fmt::Debug;
use super::Token;
use super::{ExpressionTrait, ExpressionObj};

impl ExpressionTrait for NumberExpression { }
impl ExpressionTrait for StringExpression { }
impl ExpressionTrait for SymbolExpression { }
impl ExpressionTrait for BinaryExpression { }
impl ExpressionTrait for PrefixExpression { }
impl ExpressionTrait for AssignmentExpression { }

#[derive(Debug)]
pub struct NumberExpression {
    pub value: i32,
}

#[derive(Debug)]
pub struct StringExpression {
    pub value: String
}

#[derive(Debug)]
pub struct SymbolExpression {
    pub value: String
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: ExpressionObj,
    pub operator: Token,
    pub right: ExpressionObj
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub sign: Token,
    pub right_hand: ExpressionObj
}

#[derive(Debug)]
pub struct AssignmentExpression {
    pub left: ExpressionObj,
    pub operator_kind: Token,
    pub right: ExpressionObj,
}