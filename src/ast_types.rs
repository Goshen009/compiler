use std::collections::HashMap;
use super::tokens::Token;

pub mod program;
pub mod statements;
pub mod expressions;
pub mod types;

pub use program::*;
pub use statements::*;
pub use expressions::*;
pub use types::*;


pub fn return_array_expression(array: Vec<ExpressionEnum>) -> ExpressionEnum {
    ExpressionEnum::Array(Box::new(
        ArrayExpression {
            array,
        }
    ))
}

pub fn return_empty_tuple() -> ExpressionEnum {
    ExpressionEnum::EmptyTuple
}

pub fn return_number_expression(value: i32) -> ExpressionEnum {
    ExpressionEnum::Number(
        NumberExpression {
            value,
        }
    )
}

pub fn return_string_expression(value: String) -> ExpressionEnum {
    ExpressionEnum::String(
        StringExpression {
            value,
        }
    )
}

pub fn return_symbol_expression(value: String) -> ExpressionEnum {
    ExpressionEnum::Symbol(
        SymbolExpression {
            value
        }
    )
}

pub fn return_binary_expression(left: ExpressionEnum, operator: Token, right: ExpressionEnum) -> ExpressionEnum {
    ExpressionEnum::Binary(Box::new(
        BinaryExpression {
            left,
            operator,
            right
        }
    ))
}

pub fn return_prefix_expression(sign: Token, right_hand: ExpressionEnum) -> ExpressionEnum {
    ExpressionEnum::Prefix(Box::new(
        PrefixExpression {
            sign,
            right_hand
        }
    ))
}

pub fn return_assignment_expression(left: ExpressionEnum, operator_kind: Token, right: ExpressionEnum) -> ExpressionEnum {
    ExpressionEnum::Assignment(Box::new(
        AssignmentExpression {
            left,
            operator_kind,
            right
        }
    ))
}

pub fn return_tuple_assignment_expression(tuple: Vec<ExpressionEnum>) -> ExpressionEnum {
    ExpressionEnum::TupleAssignment(Box::new(
        TupleAssignmentExpression {
            tuple
        }
    ))
}

pub fn return_struct_assignment_expression(struct_name: String, struct_fields: HashMap<String, ExpressionEnum>) -> ExpressionEnum {
    ExpressionEnum::StructAssignment(Box::new(
        StructAssignmentExpression {
            struct_name,
            struct_fields
        }
    ))
}

pub fn return_function_call_expression(function_name: String, function_argument: ExpressionEnum) -> ExpressionEnum {
    ExpressionEnum::FunctionCall(Box::new(
        FunctionCallExpression {
            function_name,
            function_argument
        }
    ))
}


pub fn return_expression_statement(expression: ExpressionEnum) -> StatementEnum {
    StatementEnum::Expression(Box::new(
        ExpressionStatement {
            expression,
        }
    ))
}

pub fn return_variable_declaration_statement(variable_name: String, is_constant: bool, value: ExpressionEnum, explicit_type: TypesEnum) -> StatementEnum {
    StatementEnum::VariableDeclaration(Box::new(
        VariableDeclarationStatement {
            variable_name,
            is_constant,
            value,
            explicit_type
        }  
    ))
}

pub fn return_tuple_declaration_statement(is_constant: bool, variable_name_and_type: Vec<(String, TypesEnum)>, value: ExpressionEnum,) -> StatementEnum {
    StatementEnum::TupleDeclaration(Box::new(
        TupleDeclarationStatement {
            is_constant,
            variable_name_and_type,
            value
        }
    ))
}

pub fn return_struct_declaration_statement(struct_name: String, struct_fields: HashMap<String, TypesEnum>) -> StatementEnum {
    StatementEnum::StructDeclaration(Box::new(
        StructDeclarationStatement {
            struct_name,
            struct_fields,
        }
    ))
}

pub fn return_function_declaration_statement(function_name: String, function_parameters: Vec<(String, TypesEnum)>, function_return_type: TypesEnum, function_body:BlockStatement ) -> StatementEnum {
    StatementEnum::FunctionDeclaration(Box::new (
        FunctionDeclarationStatement {
            function_name,
            function_parameters,
            function_return_type,
            function_body,
        }
    ))
}

pub fn return_primary_type(type_: Types) -> TypesEnum {
    TypesEnum::Primary(
        PrimaryType {
            type_,
        }
    )
}

pub fn return_array_type(underlying: TypesEnum) -> TypesEnum {
    TypesEnum::Array(Box::new(
        ArrayType {
            underlying,
        }
    ))
}

pub fn return_tuple_type(tuple: Vec<TypesEnum>) -> TypesEnum {
    TypesEnum::Tuple(Box::new(
        TupleType {
            tuple,
        }
    ))
}

pub fn return_empty_tuple_type() -> TypesEnum {
    return_tuple_type(Vec::new())
}