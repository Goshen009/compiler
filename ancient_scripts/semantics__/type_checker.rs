use super::*;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TypeCheckErrors {
    Assigned_Wrong_Value,
    Symbol_Does_Not_Exist,
    Struct_Does_Not_Exist,
    Struct_Fields_Do_Not_Match,
    Function_Does_Not_Exist,
}

// what was I trying to do here bayii?! Har har har
fn check_if_explicit_type_exists<'a>(type_to_check: &TypesEnum, curr_scope: &Scope<'a>) -> bool {
    true
    // I think this was to help with the tuple ni sha! Hmmmmmm
}

pub fn type_check<'a>(type_to_check: &TypesEnum, assigned_value: &ExpressionEnum, curr_scope: &Scope<'a>) -> Result<(), Vec<TypeCheckErrors>> {
    let result = check_if_explicit_type_exists(type_to_check, curr_scope);
    if result == false {
        // returns an error
    }
    
    match assigned_value {
        ExpressionEnum::EmptyTuple => empty_tuple_check(type_to_check),
        ExpressionEnum::Number(_) => number_check(type_to_check),
        ExpressionEnum::String(_) => string_check(type_to_check),
        ExpressionEnum::Symbol(sym_expr) => symbol_check(sym_expr, type_to_check, curr_scope),
        ExpressionEnum::Array(arr_expr) => array_check(arr_expr, type_to_check, curr_scope),
        ExpressionEnum::Binary(bin_expr) => binary_check(bin_expr, type_to_check, curr_scope),
        ExpressionEnum::Prefix(pre_expr) => prefix_check(pre_expr, type_to_check, curr_scope),
        ExpressionEnum::Assignment(ass_expr) => todo!(),
        ExpressionEnum::TupleAssignment(tup_expr) => tuple_check(tup_expr, type_to_check, curr_scope),
        ExpressionEnum::StructAssignment(strt_expr) => struct_check(strt_expr, type_to_check, curr_scope),
        ExpressionEnum::FunctionCall(func_call_expr) => function_call_check(func_call_expr, type_to_check, curr_scope),
        _ => panic!()
    }
}

fn function_call_check<'a>(func_call_expr: &Box<FunctionCallExpression>, type_to_check: &TypesEnum, curr_scope: &Scope<'a>) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: Number = get_number(e, f);

    let symbol = curr_scope.check_if_symbol_is_defined_in_scope(&func_call_expr.function_name);
    if symbol == None {
        return Err(vec![TypeCheckErrors::Function_Does_Not_Exist]);
    }

    if let Symbol::Function{parameters, return_type} = symbol.unwrap() {
        if type_to_check != *return_type {
            return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
        }

        println!("I'll have to do the errors on function calls properly!");
        let result = type_check(parameters, &func_call_expr.function_argument, curr_scope);
        return result;
    }
    return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
}

fn function_call_cheadfasck<'a>(func_call_expr: &Box<FunctionCallExpression>, type_to_check: &TypesEnum, curr_scope: &Scope<'a>) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: Number = get_number(e, f);

    let symbol = curr_scope.check_if_symbol_is_defined_in_scope(&func_call_expr.function_name);
    if symbol == None {
        return Err(vec![TypeCheckErrors::Function_Does_Not_Exist]);
    }

    if let Symbol::Function{parameters, return_type} = symbol.unwrap() {
        if type_to_check != *return_type {
            return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
        }



        // Now check that each of the parameters assigned to the function are correct
        // Can i just use type check?



        // let mut errors: Vec<TypeCheckErrors> = Vec::new();
        // for value in arr_expr.array.iter() {
        //     let result = type_check(&underlying.underlying, value, curr_scope);
        //     if result.is_err() {
        //         errors.append(&mut result.unwrap_err());
        //     }
        // }

        // if errors.is_empty() { return Ok(()); }
        // else { return Err(errors); }
    }
    return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
    
    
    // use the symbol of the function to check
    // I'm pretty tired now!


    // if let TypesEnum::Tuple(tuple) = type_to_check {
    //     if tuple.tuple.len() == 0 { // An empty tuple would have a nothing in it's Vec
    //         return Ok(());
    //     }
    //     return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
    // }
    // return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
}

fn struct_check<'a>(strt_expr: &Box<StructAssignmentExpression>, type_to_check: &TypesEnum, curr_scope: &Scope<'a>) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: MyStruct = { field1: 90, field2: b };
    if type_to_check != &return_primary_type(Types::User_Defined(return_hash(&strt_expr.struct_name))) {
        return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
    }

    let symbol = curr_scope.check_if_symbol_is_defined_in_scope(&strt_expr.struct_name);
    if symbol == None {
        return Err(vec![TypeCheckErrors::Struct_Does_Not_Exist]);
    }

    if let Symbol::Struct(sym_fields) = symbol.unwrap() {
        if sym_fields.len() != strt_expr.struct_fields.len() { // Number of fields in the struct and the defination do not match.
            return Err(vec![TypeCheckErrors::Struct_Fields_Do_Not_Match]);
        }

        let sym_keys: HashSet<_> = sym_fields.keys().collect();
        let expr_keys: HashSet<_> = strt_expr.struct_fields.keys().collect();

        if sym_keys != expr_keys { // Not all keys in the defination are in the instantiation of the struct.
            return Err(vec![TypeCheckErrors::Struct_Fields_Do_Not_Match]);
        }

        let mut errors = Vec::new();

        for (sym_name, sym_type) in sym_fields.iter() {
            let value = strt_expr.struct_fields.get(sym_name);
            if value == None { // This shouldn't happen cuz we've already checked the keys... But this error would mean that there's a field in the struct that isn't in the definition.
                return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
            }

            let result = type_check(sym_type, value.unwrap(), curr_scope);
            if result.is_err() {
                errors.append(&mut result.unwrap_err());
            }
        }

        if errors.is_empty() { return Ok(()); }
        else { return Err(errors); }
    }
    return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
}

fn tuple_check<'a>(tup_expr: &Box<TupleAssignmentExpression>, type_to_check: &TypesEnum, curr_scope: &Scope<'a>) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: Number = (90, 95); or let (a: Number, b: Number) = (90, 95);

    if let TypesEnum::Tuple(tuple) = type_to_check {
        let mut errors: Vec<TypeCheckErrors> = Vec::new();

        // first check that the number of values in the type is equal to the number of values in the expression
        if tuple.tuple.len() != tup_expr.tuple.len() {
            return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
        }

        for i in 0..tuple.tuple.len() {
            let result = type_check(&tuple.tuple[i], &tup_expr.tuple[i], curr_scope);
            if result.is_err() {
                errors.append(&mut result.unwrap_err());
            }
        }

        if errors.is_empty() { return Ok(()); }
        else { return Err(errors); }
    }
    return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
}

fn prefix_check<'a>(pre_expr: &Box<PrefixExpression>, type_to_check: &TypesEnum, curr_scope: &Scope<'a>) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: Number = -90;
    let result = type_check(type_to_check, &pre_expr.right_hand, curr_scope);
    return result;
}

fn binary_check<'a>(bin_expr: &Box<BinaryExpression>, type_to_check: &TypesEnum, curr_scope: &Scope<'a>) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: Number = 90 * 50;
    let left_bool = type_check(type_to_check, &bin_expr.left, curr_scope);
    let right_bool = type_check(type_to_check, &bin_expr.right, curr_scope);

    let mut errors = Vec::new();
    if left_bool.is_err() {
        errors.append(&mut left_bool.unwrap_err());
    }

    if right_bool.is_err() {
        errors.append(&mut right_bool.unwrap_err());
    }

    if errors.is_empty() { return Ok(()); }
    else { return Err(errors); }
}

fn array_check<'a>(arr_expr: &Box<ArrayExpression>, type_to_check: &TypesEnum, curr_scope: &Scope<'a>) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: []Number = [90, 50];
    if let TypesEnum::Array(underlying) = type_to_check {
        let mut errors: Vec<TypeCheckErrors> = Vec::new();
        for value in arr_expr.array.iter() {
            let result = type_check(&underlying.underlying, value, curr_scope);
            if result.is_err() {
                errors.append(&mut result.unwrap_err());
            }
        }

        if errors.is_empty() { return Ok(()); }
        else { return Err(errors); }
    }
    return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
}

fn symbol_check<'a>(sym_expr: &SymbolExpression, type_to_check: &TypesEnum, curr_scope: &Scope<'a>) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: Number = cd;
    let symbol = curr_scope.check_if_symbol_is_defined_in_scope(&sym_expr.value);
    if symbol == None {
        return Err(vec![TypeCheckErrors::Symbol_Does_Not_Exist]); // I'm unsure of how to go about this but it's supposed to return an error that says the symbol does not exist within the scope. But then, if we say it's assigned the wrong type, that should still cover it.
    }

    if let Symbol::Variable(sym_type) = symbol.unwrap() {
        if *sym_type == type_to_check {
            return Ok(());
        }
        return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
    }
    return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
}

fn string_check(type_to_check: &TypesEnum) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: String = "Hello!"
    if let TypesEnum::Primary(type_) = type_to_check {
        if let Types::String = type_.type_ {
            return Ok(());
        }
        return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
    }
    return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
}

fn number_check(type_to_check: &TypesEnum) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: Number = 90;
    if let TypesEnum::Primary(type_) = type_to_check {
        if let Types::Number = type_.type_ {
            return Ok(());
        }
        return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
    }
    return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
}

fn empty_tuple_check(type_to_check: &TypesEnum) -> Result<(), Vec<TypeCheckErrors>> {
    // let a: () = ();
    if let TypesEnum::Tuple(tuple) = type_to_check {
        if tuple.tuple.len() == 0 { // An empty tuple would have a nothing in it's Vec
            return Ok(());
        }
        return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
    }
    return Err(vec![TypeCheckErrors::Assigned_Wrong_Value]);
}