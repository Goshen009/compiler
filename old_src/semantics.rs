use std::collections::HashMap;
use crate::{errorq::SemanticsError, ast_types::Program, parser::Parser};
use super::ast_types::{return_tuple_type, statements::*, types::*};

pub mod objects;
mod type_checker;

use objects::*;
use type_checker::*;

pub fn start(program: &Program, mut global_scope: Scope) -> SemanticsError {
    let mut semantics_error = SemanticsError::new();

    for stmt in program.code.body.iter() {
        check_statement_semantics(stmt, &mut global_scope, &mut semantics_error);       
    }

    return semantics_error;
}

fn check_statement_semantics(stmt: &StatementEnum, scope: &mut Scope, errors: &mut SemanticsError) {
    match stmt {
        StatementEnum::FunctionDeclaration(func) => {
            if scope.is_global_scope() {
                let mut function_scope = Scope::new(Some(scope));

                for (param_name, param_type) in func.function_parameters.iter() {
                    if function_scope.has_name_been_used_before(param_name) {
                        errors.add_error(format!("There is already a function, struct or variable with the name '{}'", param_name));
                        break;
                    }
                    function_scope.scope.insert(param_name.clone(), Symbol::Variable(param_type.clone()));
                }

                for stmt in func.function_body.body.iter() {
                    check_statement_semantics(stmt, &mut function_scope, errors);      
                }
            } else {
                errors.add_error(format!("You cannot create a function inside a function"));
           }
        }
        StatementEnum::StructDeclaration(strt) => {
            if !scope.is_global_scope() {
                if scope.has_name_been_used_before(&strt.struct_name) {
                    errors.add_error(format!("There is already a function, struct or variable with the name '{}'", &strt.struct_name));
                }

                scope.scope.insert(strt.struct_name.clone(), Symbol::new_struct(&strt));
            }
        }
        StatementEnum::TupleDeclaration(tup) => {
            // the thought - convert the variable types in it to a single tuple type enum and then check with that... like what we do for functions in scope.
            let mut tuple: Vec<TypesEnum> = tup.variable_name_and_type.iter().map(|(_, param_type)| param_type.clone()).collect();
            let tuple_type = if tuple.len() == 1 { tuple.pop().unwrap() } else { return_tuple_type(tuple) };

            let type_check_result = type_check(&tuple_type, &tup.value, scope);

            if !scope.is_global_scope() {
                let mut index = 0;

                for (name, _) in tup.variable_name_and_type.iter() {
                    if scope.has_name_been_used_before(name) {
                        errors.add_error(format!("There is already a function, struct or variable with the name '{}'", name));
                    }
        
                    scope.scope.insert(name.clone(), Symbol::new_variable_from_tuple(&tup, index));
                    index += 1;
                }
            }

            if type_check_result.is_err() {
                let err_msg = format!("{}'{}'", type_check_result.unwrap_err(), "tuple somewhere jare");
                errors.add_error(err_msg);
            }
        }
        StatementEnum::VariableDeclaration(var) => {
            let type_check_result = type_check(&var.explicit_type, &var.value, scope);

            if !scope.is_global_scope() {
                if scope.has_name_been_used_before(&var.variable_name) {
                    errors.add_error(format!("There is already a function, struct or variable with the name '{}'", &var.variable_name));
                }

                scope.scope.insert(var.variable_name.clone(), Symbol::new_variable(&var));
            }

            if type_check_result.is_err() {
                let err_msg = format!("{}'{}'", type_check_result.unwrap_err(), var.variable_name.as_str());
                errors.add_error(err_msg);
            }
        }
    }
}

impl StatementEnum {
    pub fn add_to_global_scope(&self, scope: &mut Scope, parser: &mut Parser) {
        // we can do checks on here to ensure that the statement is in the global scope or the inner scope.
        match self {
            StatementEnum::FunctionDeclaration(func) => {
                if scope.has_name_been_used_before(&func.function_name) {
                    parser.errors.add_error(format!("There is already a function, struct or variable with the name '{}'", &func.function_name));
                } else {
                    scope.scope.insert(func.function_name.clone(), Symbol::new_function(&func));
                }
            }

            StatementEnum::StructDeclaration(strt) => {
                if scope.has_name_been_used_before(&strt.struct_name) {
                    parser.errors.add_error(format!("There is already a function, struct or variable with the name '{}'", &strt.struct_name));
                } else {
                    scope.scope.insert(strt.struct_name.clone(), Symbol::new_struct(&strt));
                }
            }

            StatementEnum::TupleDeclaration(tup) => {
                for i in 0..tup.variable_name_and_type.len() {
                    let name = &tup.variable_name_and_type[i].0;

                    if scope.has_name_been_used_before(name) {
                        parser.errors.add_error(format!("There is already a function, struct or variable with the name '{}'", name));
                    } else {
                        scope.scope.insert(name.clone(), Symbol::new_variable_from_tuple(&tup, i));
                    }
                }
            }

            StatementEnum::VariableDeclaration(var) => {
                if scope.has_name_been_used_before(&var.variable_name) {
                    parser.errors.add_error(format!("There is already a function, struct or variable with the name '{}'", &var.variable_name));
                } else {
                    scope.scope.insert(var.variable_name.clone(), Symbol::new_variable(&var));
                }
            }
        }
    }
}