use std::collections::HashSet;
use super::{Scope, Symbol, super::ast_types::*};

pub fn type_check(type_to_check: &TypesEnum, assigned_value: &ExpressionEnum, curr_scope: &Scope) -> Result<(), String> {
    match assigned_value {
        ExpressionEnum::EmptyTuple => empty_tuple_check(type_to_check),
        ExpressionEnum::Number(_) => number_check(type_to_check),
        ExpressionEnum::String(_) => string_check(type_to_check),
        ExpressionEnum::Symbol(sym_expr) => symbol_check(sym_expr, type_to_check, curr_scope),
        ExpressionEnum::Array(arr_expr) => array_check(arr_expr, type_to_check, curr_scope),
        ExpressionEnum::Binary(bin_expr) => binary_check(bin_expr, type_to_check, curr_scope),
        ExpressionEnum::Prefix(pre_expr) => prefix_check(pre_expr, type_to_check, curr_scope),
        ExpressionEnum::TupleAssignment(tup_expr) => tuple_check(tup_expr, type_to_check, curr_scope),
        ExpressionEnum::StructAssignment(strt_expr) => struct_check(strt_expr, type_to_check, curr_scope),
        ExpressionEnum::FunctionCall(func_call_expr) => function_call_check(func_call_expr, type_to_check, curr_scope),
    }
}

fn function_call_check(func_call_expr: &Box<FunctionCallExpression>, type_to_check: &TypesEnum, curr_scope: &Scope) -> Result<(), String> {
    // let a: Number = get_number(e, f);
    let symbol = curr_scope.is_symbol_defined_in_scope(&func_call_expr.function_name);
    if symbol == None {
        return Err(format!("No function has been declared with the name ")); // the error name will be wrong sha
    }
    
    if let Symbol::Function{parameters, return_type} = symbol.unwrap() {
        if type_to_check != return_type {
            return Err(format!("A wrong value was assigned to "));
        }

        let result = type_check(parameters, &func_call_expr.function_argument, curr_scope);
        if result.is_err() {
            return Err(format!("The function call was assigned wrong values in variable "));
        }

        return Ok(());
    }

    return Err(format!("A wrong value was assigned to "));
}

fn struct_check(strt_expr: &Box<StructAssignmentExpression>, type_to_check: &TypesEnum, curr_scope: &Scope) -> Result<(), String> {
    // let a: MyStruct = { field1: 90, field2: b };
    if type_to_check != &return_primary_type(Types::User_Defined(strt_expr.struct_name.clone())) {
        return Err(format!("A wrong value was assigned to "));
    }

    let symbol = curr_scope.is_symbol_defined_in_scope(&strt_expr.struct_name);
    if symbol == None {
        return Err(format!("No struct has been declared with the type given to variable "));
    }

    if let Symbol::Struct(sym_fields) = symbol.unwrap() {
        // I'll use a hashset to check if all the fields in the struct definition are present in the variable initialization.
        let sym_keys: HashSet<&String> = sym_fields.keys().collect();
        let expr_keys: HashSet<&String> = strt_expr.struct_fields.keys().collect();

        if sym_keys != expr_keys { // Not all keys in the defination are in the instantiation of the struct.
            return Err(format!("Check the fields of the struct assigned to "));
        }

        for (sym_name, sym_type) in sym_fields.iter() {
            let value = strt_expr.struct_fields.get(sym_name);
            if value == None { // This shouldn't happen cuz we've already checked the keys... But this error would mean that there's a field in the struct that isn't in the definition.
                return Err(format!("Check the fields of the struct assigned to "));
            }

            let result = type_check(sym_type, value.unwrap(), curr_scope);
            if result.is_err() {
                return Err(format!("A wrong value was assigned to "));
            }
        }

        return Ok(());
    }

    return Err(format!("A wrong value was assigned to "));
}

fn tuple_check(tup_expr: &Box<TupleAssignmentExpression>, type_to_check: &TypesEnum, curr_scope: &Scope) -> Result<(), String> {
    // let a: (Number, Number) = (90, 95);
    if let TypesEnum::Tuple(tuple) = type_to_check {
        // first check that the number of values in the type is equal to the number of values in the expression
        if tuple.tuple.len() != tup_expr.tuple.len() {
            return Err(format!("A wrong value was assigned to "));
        }

        for i in 0..tuple.tuple.len() {
            let result = type_check(&tuple.tuple[i], &tup_expr.tuple[i], curr_scope);
            if result.is_err() { // if any value in the tupe is wrong e.g 'see array', everything will fail the type test
                return Err(format!("A wrong value was assigned to "));
            }
        }

        return Ok(());
    }

    return Err(format!("A wrong value was assigned to "));
}

fn prefix_check(pre_expr: &Box<PrefixExpression>, type_to_check: &TypesEnum, curr_scope: &Scope) -> Result<(), String> {
    // let a: Number = -90;
    let result = type_check(type_to_check, &pre_expr.right_hand, curr_scope);
    if result.is_ok() {
        return Ok(());
    }

    return Err(format!("A wrong value was assigned to "));
}

fn binary_check(bin_expr: &Box<BinaryExpression>, type_to_check: &TypesEnum, curr_scope: &Scope) -> Result<(), String> {
    // let a: Number = 90 * 50;
    let left_expr = type_check(type_to_check, &bin_expr.left, curr_scope);
    let right_expr = type_check(type_to_check, &bin_expr.right, curr_scope);

    if left_expr.is_ok() && right_expr.is_ok() {
        return Ok(());
    }

    return Err(format!("A wrong value was assigned to "));
}

fn array_check(arr_expr: &Box<ArrayExpression>, type_to_check: &TypesEnum, curr_scope: &Scope) -> Result<(), String> {
    // let a: []Number = [90, 50];
    if let TypesEnum::Array(underlying) = type_to_check {
        for value in arr_expr.array.iter() {
            let result = type_check(&underlying.underlying, value, curr_scope);
            if result.is_err() { // if any value in the array is wrong e.g [90, "50"], everything will fail the type test
                return Err(format!("A wrong value was assigned to "));
            }
        }

        return Ok(());
    }

    return Err(format!("A wrong value was assigned to "));
}

fn symbol_check(sym_expr: &SymbolExpression, type_to_check: &TypesEnum, curr_scope: &Scope) -> Result<(), String> {
    // let a: Number = sym;
    let symbol = curr_scope.is_symbol_defined_in_scope(&sym_expr.value);
    if symbol == None {
        return Err(format!("No symbol has been declared with the value given to variable "));
    }

    if let Symbol::Variable(sym_type) = symbol.unwrap() {
        if sym_type == type_to_check {
            return Ok(());
        }
    }

    return Err(format!("A wrong value was assigned to "));
}

fn string_check(type_to_check: &TypesEnum) -> Result<(), String> {
    // let a: Sting = "Hello";
    if let TypesEnum::Primary(typeq) = type_to_check {
        if let Types::String = typeq.typeq {
            return Ok(());
        }
    }

    return Err(format!("A wrong value was assigned to "));
}

fn number_check(type_to_check: &TypesEnum) -> Result<(), String> {
    // let a: Number = 90;
    if let TypesEnum::Primary(typeq) = type_to_check {
        if let Types::Number = typeq.typeq {
            return Ok(());
        }
    }

    return Err(format!("A wrong value was assigned to "));
}

fn empty_tuple_check(type_to_check: &TypesEnum) -> Result<(), String> {
    // let a: () = ();
    if let TypesEnum::Tuple(tuple) = type_to_check {
        if tuple.tuple.len() == 0 {
            return Ok(());
        }
    }

    return Err(format!("A wrong value was assigned to "));
}