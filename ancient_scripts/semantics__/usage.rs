use super::*;









// pub fn add_symbol_to_scope<'a>(statement: &'a State)

pub fn create_symbol<'a>(statement: &'a StatementEnum, global_scope: &mut Scope<'a>) -> Vec<ErrorCodes> {
    match statement {
        StatementEnum::Block(_) => vec![],
        StatementEnum::FunctionDeclaration(func) => {
            if global_scope.has_variable_name_been_used_before(&func.function_name) == true {
                return vec![ErrorCodes::Function_Name_Has_Been_Used_Before(func.function_name.clone())];
            }

            let function_symbol = return_function_symbol(&func.function_parameters, &func.function_return_type);
            global_scope.scope.insert(&func.function_name, function_symbol);
            return vec![];
        },
        StatementEnum::StructDeclaration(strt) => {
            if global_scope.has_variable_name_been_used_before(&strt.struct_name) == true {
                return vec![ErrorCodes::Struct_Name_Has_Been_Used_Before(strt.struct_name.clone())];
            }
        
            let struct_symbol = return_struct_symbol(&strt.struct_fields);
            global_scope.scope.insert(&strt.struct_name, struct_symbol);
        
            return vec![];
        },
        StatementEnum::TupleDeclaration(tup) => {
            let mut errors: Vec<ErrorCodes> = Vec::new();
            for (name, type_) in tup.variable_name_and_type.iter() {
                if global_scope.has_variable_name_been_used_before(name) == true {
                    errors.append(&mut vec![ErrorCodes::Variable_Name_Has_Been_Used_Before(name.clone())]);
                }

                let symbol = return_variable_symbol(type_);
                global_scope.scope.insert(name, symbol);
            }

            return errors;
        }
        StatementEnum::VariableDeclaration(var) => {
            if global_scope.has_variable_name_been_used_before(&var.variable_name) == true {
                return vec![ErrorCodes::Variable_Name_Has_Been_Used_Before(var.variable_name.clone())];
            }
            
            let var_symbol = return_variable_symbol(&var.explicit_type);
            global_scope.scope.insert(&var.variable_name, var_symbol);
            
            return vec![];
        },
        _ => todo!()
    }
}

pub fn check_statement<'a>(statement: &'a StatementEnum, curr_scope: &mut Scope<'a>) -> Vec<ErrorCodes> {
    match statement {
        StatementEnum::Block(block) => check_block_statement(block, curr_scope),
        StatementEnum::FunctionDeclaration(func) => check_function_statement(func, curr_scope),
        StatementEnum::StructDeclaration(strt) => check_struct_statement(strt, curr_scope),
        StatementEnum::VariableDeclaration(var) => check_variable_statement(var, curr_scope),
        StatementEnum::TupleDeclaration(tup) => check_tuple_statement(tup, curr_scope),
        _ => todo!()
    }
}

fn check_tuple_statement<'a>(tup: &'a Box<TupleDeclarationStatement>, curr_scope: &mut Scope<'a>) -> Vec<ErrorCodes> {
    if tup.variable_name_and_type.len() == 0 {
        // let () = sumn; Throw an error jare. I'm not allowing it.
        return vec![ErrorCodes::Empty_Tuple_Defined()];
    }

    // Start by checking that each of the variable names have not been used before in scope. Then create a tuple type which will be used in the type checker.
    let mut tuple: Vec<TypesEnum> = Vec::new();

    for (name, type_) in tup.variable_name_and_type.iter() {
        if curr_scope.has_variable_name_been_used_before(name) == true {
            return vec![ErrorCodes::Variable_Name_Has_Been_Used_Before(name.clone())];
        }

        tuple.push(type_.clone());
    }

    let tuple_enum =  return_tuple_type(tuple);
    let type_check_result = type_check(&tuple_enum, &tup.value, curr_scope);

    // Check on the types first before adding to the scope. Had an issue where I assigned a variable to itself.
    for (name, type_) in tup.variable_name_and_type.iter() {
        let var_symbol = return_variable_symbol(type_);
        curr_scope.scope.insert(name, var_symbol);
    }

    // return check_result(type_check_result, );
    println!("there's a likkle issue here! The variable uses a function to turn it into error codes, but I can't use that because this is a tuple.");

    if type_check_result.is_err() {
        println!("All errors tuple: {:?}", type_check_result.err().unwrap());
    }

    return Vec::new();
}

fn check_variable_statement<'a>(var: &'a Box<VariableDeclarationStatement>, curr_scope: &mut Scope<'a>) -> Vec<ErrorCodes> {
    // Start by checking that the variable name has not been used before in scope. Then add the variable's symbol to the scope. Finally check if what was assigned is the right type
    if curr_scope.has_variable_name_been_used_before(&var.variable_name) == true {
        return vec![ErrorCodes::Variable_Name_Has_Been_Used_Before(var.variable_name.clone())];
    }

    let type_check_result = type_check(&var.explicit_type, &var.value, curr_scope);

    // Check on the types first before adding to the scope. Had an issue where I assigned a variable to itself.
    let var_symbol = return_variable_symbol(&var.explicit_type);
    curr_scope.scope.insert(&var.variable_name, var_symbol);
    
    return check_result(type_check_result, var);
}

fn check_result<'a>(result: Result<(), Vec<TypeCheckErrors>>, var: &'a Box<VariableDeclarationStatement>) -> Vec<ErrorCodes> {
    if result.is_ok() {
        return vec![];
    }

    let mut error_codes: Vec<ErrorCodes> = Vec::new();
    for error in result.unwrap_err() {
        match error {
            TypeCheckErrors::Assigned_Wrong_Value => error_codes.push(ErrorCodes::Variable_Is_Assigned_An_Incorrect_Type(var.variable_name.clone())),
            TypeCheckErrors::Symbol_Does_Not_Exist => error_codes.push(ErrorCodes::Assigned_Symbol_Does_Not_Exist()),
            TypeCheckErrors::Struct_Does_Not_Exist => error_codes.push(ErrorCodes::Struct_Not_Defined()),
            TypeCheckErrors::Struct_Fields_Do_Not_Match => error_codes.push(ErrorCodes::Struct_Fields_Do_Not_Match()),
            TypeCheckErrors::Function_Does_Not_Exist => error_codes.push(ErrorCodes::Assigned_Symbol_Does_Not_Exist()),
        }
    }

    return error_codes;
}

fn check_struct_statement<'a>(strt: &'a Box<StructDeclarationStatement>, curr_scope: &mut Scope<'a>) -> Vec<ErrorCodes> {
    // We check that the name of the struct has not been used before in scope. Then adds a struct symbol to the scope.
    if curr_scope.has_variable_name_been_used_before(&strt.struct_name) == true {
        return vec![ErrorCodes::Struct_Name_Has_Been_Used_Before(strt.struct_name.clone())];
    }

    let struct_symbol = return_struct_symbol(&strt.struct_fields);
    curr_scope.scope.insert(&strt.struct_name, struct_symbol);

    return vec![];
}

fn check_function_statement<'a>(func: &Box<FunctionDeclarationStatement>, parent_scope: &Scope<'a>) -> Vec<ErrorCodes> {
    let mut function_scope = Scope {
        scope: HashMap::new(),
        parent: Some(parent_scope)
    };

    // Add the function parameters to the scope of this function. Creates symbols for them as well.
    for (parameter_name, parameter_type) in func.function_parameters.iter() {
        // Shadowing is not allowed. So the parameter names are first checked to ensure they haven't been used before.
        if function_scope.has_variable_name_been_used_before(parameter_name) == true {
            return vec![
                ErrorCodes::Function_Parameter_Has_A_Name_That_Has_Been_Used_In_Scope{
                    function_name: func.function_name.clone(), 
                    parameter_name: parameter_name.clone()
            }];
        }

        let parameter_symbol = return_variable_symbol(parameter_type);
        function_scope.scope.insert(parameter_name, parameter_symbol);
    }

    let mut function_errors: Vec<ErrorCodes> = Vec::new();
    for statement in func.function_body.body.iter() {
        function_errors.append(&mut check_statement(statement, &mut function_scope));
    }

    return function_errors;
}

fn check_block_statement<'a>(block: &Box<BlockStatement>, parent_scope: &Scope<'a>) -> Vec<ErrorCodes> {
    let mut block_scope = Scope {
        scope: HashMap::new(),
        parent: Some(parent_scope)
    };

    let mut block_errors: Vec<ErrorCodes> = Vec::new();
    for statement in block.body.iter() {
        block_errors.append(&mut check_statement(statement, &mut block_scope));
    }

    return block_errors;
}