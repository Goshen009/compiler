/* THIS IS MIMICKING OOP IN OTHER LANGUAGES */

use std::fmt::Debug;
use super::Token;

pub use statements::*;
pub use expressions::*;
pub use types::*;

pub mod statements;
pub mod expressions;
pub mod types;

pub trait ExpressionTrait: Debug { }
pub type ExpressionObj = Box<dyn ExpressionTrait>;

pub trait StatementTrait: Debug { }
pub type StatementObj = Box<dyn StatementTrait>;

pub trait TypeTrait: Debug { }
pub type TypeObj = Box<dyn TypeTrait>;

pub trait AnyTrait: Any {
    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}
impl <T: Any> AnyTrait for T {
    fn as_any(self: Box<Self>) -> Box<dyn Any>{
        self
    }
}

pub fn return_number_expression(value: i32) -> ExpressionObj {
    Box::new(
        NumberExpression {
            value,
        }
    )
}

pub fn return_string_expression(value: String) -> ExpressionObj {
    Box::new(
        StringExpression {
            value,
        }
    )
}

pub fn return_symbol_expression(value: String) -> ExpressionObj {
    Box::new(
        SymbolExpression {
            value
        }
    )
}

pub fn return_binary_expression(left: ExpressionObj, operator: Token, right: ExpressionObj) -> ExpressionObj {
    Box::new(
        BinaryExpression {
            left,
            operator,
            right
        }
    )
}

pub fn return_prefix_expression(sign: Token, right_hand: ExpressionObj) -> ExpressionObj {
    Box::new(
        PrefixExpression {
            sign,
            right_hand
        }
    )
}

pub fn return_assignment_expression(left: ExpressionObj, operator_kind: Token, right: ExpressionObj) -> ExpressionObj {
    Box::new(
        AssignmentExpression {
            left,
            operator_kind,
            right
        }
    )
}


pub fn return_expression_statement(expression: ExpressionObj) -> StatementObj {
    Box::new(
        ExpressionStatement {
            expression,
        }
    )
}

pub fn return_variable_declaration_statement(variable_name: String, is_constant: bool, value: ExpressionObj, explicit_type: Option<TypeObj>) -> StatementObj {
    Box::new(
        VariableDeclarationStatement {
            variable_name,
            is_constant,
            value,
            explicit_type
        }  
    )
}

pub fn return_primary_type(type_: String) -> TypeObj {
    Box::new(
        PrimaryType {
            type_,
        }
    )
}

pub fn return_array_type(underlying: TypeObj) -> TypeObj {
    Box::new(
        ArrayType {
            underlying,
        }
    )
}