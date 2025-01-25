use super::*;


fn create_entry_struct_symbol(strt: &Box<StructDeclarationStatement>) -> Symbol {
    Symbol::Struct(
        &strt.struct_fields
    )
}

fn create_entry_variable_symbol(var: &Box<VariableDeclarationStatement>) -> Symbol {
    Symbol::Variable(
        &var.explicit_type
    )
}

pub fn get_global_scope<'a>(ast: &'a BlockStatement) -> Scope<'a> {
    let mut global_scope = Scope {
        scope: HashMap::new(),
        parent: None
    };

    for block in ast.body.iter() {
        match block {
            StatementEnum::StructDeclaration(strt) => {
                if has_variable_name_been_used_before(&global_scope, &strt.struct_name) == false {
                    let strt_symbol = create_entry_struct_symbol(strt);
                    global_scope.scope.insert(&strt.struct_name, strt_symbol);
                } else {
                    panic!("This struct name has been used before!");
                }
            },
            StatementEnum::VariableDeclaration(var) => {
                if has_variable_name_been_used_before(&global_scope, &var.variable_name) == false {
                    let var_symbol = create_entry_variable_symbol(var);                    
                    global_scope.scope.insert(&var.variable_name, var_symbol);
                } else {
                    panic!("This variable name has been used before!");
                }
            }
            _ => todo!()
        }
    }

    global_scope
}


pub fn check_block_scope<'a>(block: &'a BlockStatement, block_scope: &mut Scope<'a>) -> bool {
    for stmt in block.body.iter() {
        let clear = match stmt {
            StatementEnum::VariableDeclaration(var) => {
                if let None = block_scope.parent { // it is in the global scope
                    println!("asdfas : {} : {:?}", var.variable_name, var.value);
                    type_check(&var.explicit_type, &var.value, block_scope);
                }

                println!("sadljfansldjkjcabwijef");
                // it is not in the global scope
                reached_variable_declaration(var, block_scope)
            }
            StatementEnum::StructDeclaration(_) => {
                println!("Struct declaration");
                true
            },
            _ => todo!()
        };

        if clear == false {
            println!("sldhfalsjkdfns");
            return false;
        }
    }

    print_scope(&block_scope.scope, "Block Scope has: ");
    return true;
}




fn reached_variable_declaration<'a>(var: &'a Box<VariableDeclarationStatement>, curr_scope: &mut Scope<'a>) -> bool {    
    if has_variable_name_been_used_before(curr_scope, &var.variable_name) == false {
        let variable_semantics_clear = type_check(&var.explicit_type, &var.value, curr_scope);
        
        if variable_semantics_clear == true {
            let var_symbol = create_entry_variable_symbol(var);
            let var_name_hash = return_hash(&var.variable_name);

            curr_scope.scope.insert(&var.variable_name, var_symbol);
        }
    
        return variable_semantics_clear;
    }
    
    return false;
}






// pub fn check_block_scope<'a>(block: &'a BlockStatement, block_scope: &mut Scope<'a>) -> bool {
//     for stmt in block.body.iter() {
//         let clear = match stmt {
//             StatementEnum::FunctionDeclaration(func) => {
//                 entered_new_function_scope(func, block_scope)
//             }
//             StatementEnum::Block(bl) => {
//                 entered_new_block_scope(bl, block_scope)
//             }
//             StatementEnum::StructDeclaration(strt) => {
//                 reached_struct_declaration(strt, block_scope)
//             }
//             StatementEnum::VariableDeclaration(var) => {
//                 if let None = block_scope.parent { // it is in the global scope
//                     return type_check(&var.explicit_type, &var.value, block_scope);
//                 }

//                 // it is not in the global scope
//                 reached_variable_declaration(var, block_scope)
//             }
//             StatementEnum::TupleDeclaration(tup) => {
//                 reached_tuple_declaration(tup, block_scope)
//             }

//             StatementEnum::Expression(_) => {
//                 // I haven't done expressions yet!
//                 panic!("I'm yet to do expressions");
//             }
//         };

//         if clear == false {
//             return false;
//         }
//     }

//     print_scope(&block_scope.scope, "Block Scope has: ");
//     return true;
// }

// pub fn get_global_scope<'a>(ast: &'a BlockStatement) -> Scope<'a> {
//     let mut global_scope = Scope {
//         scope: HashMap::new(),
//         parent: None
//     };

//     for block in ast.body.iter() {
//         match block {
//             StatementEnum::FunctionDeclaration(func) => {
//                 let func_symbol = create_entry_function_symbol(func);
//                 let func_name_hash = return_hash(&func.function_name);

//                 global_scope.scope.insert(func_name_hash, func_symbol);
//             }
//             StatementEnum::StructDeclaration(strt) => {
//                 let strt_symbol = create_entry_struct_symbol(strt);
//                 let strt_name_hash = return_hash(&strt.struct_name);

//                 global_scope.scope.insert(strt_name_hash, strt_symbol);
//             }
//             StatementEnum::VariableDeclaration(var) => {                
//                 // For variables, a problem can arise when you declare a variable after another one but try to use second variable in the first. Thus extra complexity.

//                 if has_variable_name_been_used_before(&global_scope, &var.variable_name) == false {
//                     let var_symbol = create_entry_variable_symbol(var);
//                     let var_name_hash = return_hash(&var.variable_name);
                    
//                     global_scope.scope.insert(var_name_hash, var_symbol);
//                 } else {
//                     panic!("This variable name has been used before!");
//                 }
//             }
//             StatementEnum::Expression(_) => panic!("You cannot have expression hanging around in the global scope"),
//             StatementEnum::Block(_) => panic!("You cannot create a new scope in the global scope"),
//             StatementEnum::TupleDeclaration(_) => panic!("I'm yet to do this!")
//         }
//     }

//     global_scope
// }


























// fn entered_new_function_scope<'a>(func: &Box<FunctionDeclarationStatement>, parent_scope: &Scope<'a>) -> bool {
//     let mut function_scope = Scope {
//         scope: HashMap::new(),
//         parent: Some(parent_scope)
//     };

//     for (param_name, param_type) in func.function_parameters.iter() {
//         if has_variable_name_been_used_before(&function_scope, param_name) == true { 
//             panic!("Your function variables can't have a name that has been used before");
//         }

//         let param_variable_symbol = Symbol::Variable(
//             VariableSymbol {
//                 type_ : param_type
//             }
//         );

//         function_scope.scope.insert(return_hash(param_name), param_variable_symbol);
//     }

//     check_block_scope(&func.function_body, &mut function_scope)
// }

// fn entered_new_block_scope<'a>(bl: &'a Box<BlockStatement>, parent_scope: &Scope<'a>) -> bool {
//     let mut new_block_scope = Scope {
//         scope: HashMap::new(),
//         parent: Some(parent_scope)
//     };

//     check_block_scope(&bl, &mut new_block_scope)
// }

// fn reached_struct_declaration<'a>(strt: &'a Box<StructDeclarationStatement>, curr_scope: &mut Scope<'a>) -> bool {
//     if let None = curr_scope.parent {
//         // then it is has already been added to the scope in the get_global_scope_method()
//         return true;
//     }

//     let struct_symbol = create_entry_struct_symbol(strt);
//     let struct_name_hash = return_hash(&strt.struct_name);

//     curr_scope.scope.insert(struct_name_hash, struct_symbol);
//     true
// }

// fn reached_variable_declaration<'a>(var: &'a Box<VariableDeclarationStatement>, curr_scope: &mut Scope<'a>) -> bool {    
//     if has_variable_name_been_used_before(curr_scope, &var.variable_name) == false {
//         let variable_semantics_clear = type_check(&var.explicit_type, &var.value, curr_scope);
        
//         if variable_semantics_clear == true {
//             let var_symbol = create_entry_variable_symbol(var);
//             let var_name_hash = return_hash(&var.variable_name);

//             curr_scope.scope.insert(var_name_hash, var_symbol);
//         }
    
//         return variable_semantics_clear;
//     }
    
//     return false;
// }

// fn reached_tuple_declaration<'a>(tup: &'a Box<TupleDeclarationStatement>, curr_scope: &mut Scope<'a>) -> bool { 
//     // match tup.value {
//     //     ExpressionEnum::FunctionCall(func_expr) => {
//     //         let symbol_option = curr_scope.check_if_symbol_is_defined_in_scope(&func_expr.function_name);

//     //         if let Some(sym_value) = symbol_option {
//     //             match sym_value {
//     //                 Symbol::Function(func_sym) => {
//     //                     // make sure the tuple and the return type have the same number of 
//     //                     if tup.variable_name_and_type.len() != func_sym.return_types.len() {
//     //                         panic!("It's not the same return types.");
//     //                     }

//     //                     let mut i = 0;
//     //                     for (var_name, var_type) in tup.variable_name_and_type.iter() {
//     //                         if var_type == &func_sym.return_types[i] {
                                
//     //                         }

//     //                         i += 1;
//     //                     }
//     //                 }
//     //                 _ => panic!("This should not be happening on tuple function")
//     //             }
//     //         } else {
//     //             return false; // the function hasn't been defined anywhere!
//     //         }

//     //         // get the symbol.
//     //     }
//     // }

//     // match tup.value {
//     //     ExpressionEnum::TupleAssignment(tup_expr) => {

//     //     },
//     //     ExpressionEnum::FunctionCall(func_expr) => {
//     //         // maybe break down the return type to a tuple assignment
//     //     },
//     //     ExpressionEnum::Symbol(sym_expr) => {
//     //         // do likewise here... hmmmm
//     //     },
//     //     _ => panic!("You've assigned a wrong value to this tuple")
//     // }
//     todo!()
// }   


// pub fn check_block_scope<'a>(block: &'a BlockStatement, block_scope: &mut Scope<'a>) -> bool {
//     for stmt in block.body.iter() {
//         let clear = match stmt {
//             StatementEnum::FunctionDeclaration(func) => {
//                 entered_new_function_scope(func, block_scope)
//             }
//             StatementEnum::Block(bl) => {
//                 entered_new_block_scope(bl, block_scope)
//             }
//             StatementEnum::StructDeclaration(strt) => {
//                 reached_struct_declaration(strt, block_scope)
//             }
//             StatementEnum::VariableDeclaration(var) => {
//                 if let None = block_scope.parent { // it is in the global scope
//                     return type_check(&var.explicit_type, &var.value, block_scope);
//                 }

//                 // it is not in the global scope
//                 reached_variable_declaration(var, block_scope)
//             }
//             StatementEnum::TupleDeclaration(tup) => {
//                 reached_tuple_declaration(tup, block_scope)
//             }

//             StatementEnum::Expression(_) => {
//                 // I haven't done expressions yet!
//                 panic!("I'm yet to do expressions");
//             }
//         };

//         if clear == false {
//             return false;
//         }
//     }

//     print_scope(&block_scope.scope, "Block Scope has: ");
//     return true;
// }


// fn create_entry_function_symbol(func: &Box<FunctionDeclarationStatement>) -> Symbol {    
//     Symbol::Function(
//         FunctionSymbol {
//             parameters : (|| {
//                 let mut parameters:Vec<&TypesEnum> = Vec::new();

//                 for (_, param_type) in func.function_parameters.iter() {
//                     parameters.push(param_type);
//                 }
//                 parameters
//             })(),
//             return_types: &func.function_return_type
//         }
//     )
// }

// // fn create_entry_struct_symbol(strt: &Box<StructDeclarationStatement>) -> Symbol {
// //     Symbol::Struct(
// //         StructSymbol {
// //             fields : (|| {
// //                 let mut fields: HashMap::<u64, &TypesEnum> = HashMap::new();

// //                 for (strt_field_name, strt_field_type) in strt.struct_fields.iter() {
// //                     let name_hash = return_hash(strt_field_name);
// //                     fields.insert(name_hash, strt_field_type);
// //                 }
// //                 fields
// //             })()
// //         }
// //     )
// // }

// // fn create_entry_variable_symbol(var: &Box<VariableDeclarationStatement>) -> Symbol {
// //     Symbol::Variable(
// //         VariableSymbol {
// //             type_ : &var.explicit_type
// //         }
// //     )
// // }

// pub fn get_global_scope<'a>(ast: &'a BlockStatement) -> Scope<'a> {
//     let mut global_scope = Scope {
//         scope: HashMap::new(),
//         parent: None
//     };

//     for block in ast.body.iter() {
//         match block {
//             StatementEnum::FunctionDeclaration(func) => {
//                 let func_symbol = create_entry_function_symbol(func);
//                 let func_name_hash = return_hash(&func.function_name);

//                 global_scope.scope.insert(func_name_hash, func_symbol);
//             }
//             StatementEnum::StructDeclaration(strt) => {
//                 let strt_symbol = create_entry_struct_symbol(strt);
//                 let strt_name_hash = return_hash(&strt.struct_name);

//                 global_scope.scope.insert(strt_name_hash, strt_symbol);
//             }
//             StatementEnum::VariableDeclaration(var) => {                
//                 // For variables, a problem can arise when you declare a variable after another one but try to use second variable in the first. Thus extra complexity.

//                 if has_variable_name_been_used_before(&global_scope, &var.variable_name) == false {
//                     let var_symbol = create_entry_variable_symbol(var);
//                     let var_name_hash = return_hash(&var.variable_name);
                    
//                     global_scope.scope.insert(var_name_hash, var_symbol);
//                 } else {
//                     panic!("This variable name has been used before!");
//                 }
//             }
//             StatementEnum::Expression(_) => panic!("You cannot have expression hanging around in the global scope"),
//             StatementEnum::Block(_) => panic!("You cannot create a new scope in the global scope"),
//             StatementEnum::TupleDeclaration(_) => panic!("I'm yet to do this!")
//         }
//     }

//     global_scope
// }