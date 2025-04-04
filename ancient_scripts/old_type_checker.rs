use super::*;

fn crude_error_handling(error: &str)  -> bool {
    println!("{error}");
    false
}

// fn check_symbol<'a>(type_to_check: &TypesEnum, sym_expr: &SymbolExpression, curr_scope: &Scope<'a>) -> bool {
//     let symbol_option = curr_scope.check_if_symbol_is_defined_in_scope(&sym_expr.value);
    
//     if let Some(sym_value) = symbol_option {
//         match sym_value {
//             Symbol::Variable(sym_val) => {
//                 return sym_val.check_type_variable(type_to_check);
//             }
//             _ => return crude_error_handling("On checking symbols, this shouldn't happen but the symbol's value isn't a variable") // if it's not a variable symbol, then you've got the wrong type.
//         }
//     } 
    
//     println!("Symbol wasn't found in scope");
//     return false; // if there is no symbol option, then it isn't in scope
// }

// fn check_struct_assignment<'a>(str_expr:&Box<StructAssignmentExpression>, curr_scope: &Scope<'a>) -> bool {
//     let symbol_option = curr_scope.check_if_symbol_is_defined_in_scope(&str_expr.struct_name);
//     if let Some(sym_value) = symbol_option {
//         match sym_value {
//             Symbol::Struct(sym_val) => {
//                 return sym_val.check_type_struct(str_expr, curr_scope);
//             },
//             _ => return false // if it's not a variable symbol, then you've got the wrong type.
//         }
//     }
    
//     return false; // if there is no symbol option, then it isn't in scope
// }

fn check_array<'a>(type_to_check: &TypesEnum, arr_expr: &Box<ArrayExpression>, curr_scope: &Scope<'a>) -> bool {
    if let TypesEnum::Array(underlying) = type_to_check {
        for arr in arr_expr.array.iter() {
            if type_check(&underlying.underlying, arr, curr_scope) == false {
                return false; // one of the values in the array does not match
            }
        }
        return true; // all the values in the array match
    }
    
    return false; // the expression given and the type expected do not match
}



fn check_symbol<'a>(type_to_check: &TypesEnum, sym_expr_value: &String, curr_scope: &Scope<'a>) -> bool {
    // This checks if the type of the symbol is the same as the type of the variable it is assigned to.
    // let a: Number = 90;
    // let b: Number = a;
    // 'a' here is what will get the check_symbol method called on.

    println!("asldfnasd");

    let symbol_option = curr_scope.check_if_symbol_is_defined_in_scope(sym_expr_value);
    if symbol_option == None {
        println!("The variable {} not found in scope", sym_expr_value);
        return false;
    }

    let s = Box::new(StructAssignmentExpression {
        struct_name: "OP".to_string(),
        struct_fields: HashMap::new()
    });

    return symbol_option.unwrap().check_if_type_matches_symbol_in_table(type_to_check, &return_empty_tuple(), &s,  curr_scope);
}

fn check_struct_assignment<'a>(type_to_check: &TypesEnum, strt_expr:&Box<StructAssignmentExpression>, curr_scope: &Scope<'a>) -> bool {
    // First check that the type of the struct is the same as the type we are checking on
    // let a: MyStruct = MyStruct { 
    //      field1: 90
    // }
    // We are first checking that ": MyStruct" = "MyStruct"

    let str_name_hash = return_hash(&strt_expr.struct_name);
    let type_check = type_to_check == &return_primary_type(Types::User_Defined(str_name_hash));

    if type_check == false {
        println!("The type of the struct assigned doesn't match the type of the struct in the declaration");
        return false;
    }

    // Next check that the fields of the structs are assigned properly.
    let symbol_option = curr_scope.check_if_symbol_is_defined_in_scope(&strt_expr.struct_name);
    if symbol_option == None {
        println!("The struct {} not found in scope", strt_expr.struct_name);
        return false;
    }

    return symbol_option.unwrap().check_if_type_matches_symbol_in_table(type_to_check, &return_empty_tuple(), strt_expr, curr_scope);
}



// fn check_struct_assignment<'a>(str_expr:&Box<StructAssignmentExpression>, curr_scope: &Scope<'a>) -> bool {
//     let symbol_option = curr_scope.check_if_symbol_is_defined_in_scope(&str_expr.struct_name);
//     if let Some(sym_value) = symbol_option {
//         match sym_value {
//             Symbol::Struct(sym_val) => {
//                 return sym_val.check_type_struct(str_expr, curr_scope);
//             },
//             _ => return false // if it's not a variable symbol, then you've got the wrong type.
//         }
//     }
    
//     return false; // if there is no symbol option, then it isn't in scope
// }

pub fn type_check<'a>(type_to_check: &TypesEnum, assigned_value: &ExpressionEnum,curr_scope: &Scope<'a>) -> bool {
    match assigned_value {
        ExpressionEnum::Symbol(sym_expr) => check_symbol(type_to_check, &sym_expr.value, curr_scope),
        ExpressionEnum::StructAssignment(strt_expr) => check_struct_assignment(type_to_check, strt_expr, curr_scope),

        ExpressionEnum::Number(_) => {
            return type_to_check == &return_primary_type(Types::Number)
        }

        ExpressionEnum::String(_) => {
            return type_to_check == &return_primary_type(Types::String)
        }

        _=> return false
    }
}




pub fn type_checksass<'a>(type_to_check: &TypesEnum, assigned_value: &ExpressionEnum, curr_scope: &Scope<'a>) -> bool {
    match assigned_value {
        ExpressionEnum::Symbol(sym_expr) => {
            check_symbol(type_to_check, &sym_expr.value, curr_scope) // A symbol is a variable name. When a variable is declared with a struct (or other user_defined) as it's value, the type will be of user defined with the hash of the name of that struct.
        },
        ExpressionEnum::StructAssignment(str_expr) => {
            let str_name_hash = return_hash(&str_expr.struct_name);
            let result = type_to_check == &return_primary_type(Types::User_Defined(str_name_hash));

            // result && check_struct_assignment(str_expr, curr_scope)
            todo!()
        }
        ExpressionEnum::Array(arr_expr) => {
            check_array(type_to_check, arr_expr, curr_scope)
        }
        ExpressionEnum::Binary(bin_expr) => {
            let left_bool = type_check(type_to_check, &bin_expr.left, curr_scope);
            let right_bool = type_check(type_to_check, &bin_expr.right, curr_scope);

            return left_bool && right_bool;
        }
        ExpressionEnum::Prefix(prefix_expr) => {
            type_check(type_to_check, &prefix_expr.right_hand, curr_scope)
        }
        ExpressionEnum::String(_) => {
            return type_to_check == &return_primary_type(Types::String)
        }
        ExpressionEnum::Number(_) => {
            return type_to_check == &return_primary_type(Types::Number)
        }
        ExpressionEnum::Assignment(_) => todo!(),
        ExpressionEnum::FunctionCall(_) => false,
        ExpressionEnum::TupleAssignment(_) => false,
        ExpressionEnum::EmptyTuple() => false,
    }
}




fn check_variable_statement<'a>(var: &'a Box<VariableDeclarationStatement>, curr_scope: &mut Scope<'a>) -> Vec<ErrorCodes> {
    // Start by checking that the variable name has not been used before in scope. Then add the variable's symbol to the scope. Finally check if what was assigned is the right type
    if curr_scope.has_variable_name_been_used_before(&var.variable_name) == true {
        return vec![ErrorCodes::Variable_Name_Has_Been_Used_Before(var.variable_name.clone())];
    }

    // We add the symbol to the scope first before doing the type check because other variables may rely on this variable in their assignments.
    let var_symbol = return_variable_symbol(&var.explicit_type);
    curr_scope.scope.insert(&var.variable_name, var_symbol);

            // if type_check(&var.explicit_type, &var.value, curr_scope) == false {
            //     return vec![ErrorCodes::Variable_Is_Assigned_An_Incorrect_Type(var.variable_name.clone())];
            // }

    let type_check_result = type_check(&var.explicit_type, &var.value, curr_scope);
    return check_result(type_check_result, var);

            // return vec![];
}