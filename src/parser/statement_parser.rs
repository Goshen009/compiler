use super::{*, handlers::BindingPower};
use std::collections::HashMap;

pub fn parse_statement(parser: &mut Parser, log_error: bool) -> Option<StatementEnum> {
    let stmt_fn = parser.get_current_token().stmt();
    if stmt_fn.is_none() {
        let error = format!("Expects a statement at {}", parser.advance().get_position());
        if log_error { parser.errors.add_error(error); }

        return None;
    }
    
    return (stmt_fn.unwrap())(parser);
}

pub fn parse_struct_declaration_statement(parser: &mut Parser) -> Option<StatementEnum> {
    parser.advance(); // we've moved past struct

    let mut struct_name_token = parser.expect(Token::SYMBOL, "Expects a name for struct")?;
    let struct_name = get_name_from_symbol(struct_name_token.take_value());

    parser.expect(Token::OPEN_CURLY, &Token::OPEN_CURLY.as_expect_error())?;
    
    let mut struct_fields: HashMap<String, TypesEnum> = HashMap::new();

    while parser.get_current_token() != Token::CLOSE_CURLY {
        let mut field_name_token = parser.expect(Token::SYMBOL, &format!("Expects a field for struct '{}'", struct_name))?;
        let field_name = get_name_from_symbol(field_name_token.take_value());

        // checks if the hashmap already has a field with that name
        if struct_fields.contains_key(&field_name) { parser.log_error(&format!("Struct '{}' already has a field with name '{}' at {}", struct_name, field_name, field_name_token.get_position()))?; }
    
        parser.expect(Token::COLON, &Token::COLON.as_expect_error())?;
        let field_type = parse_type(parser)?;

        let is_final_iteration = parser.get_current_token() == Token::CLOSE_CURLY;
        if !is_final_iteration {
            parser.expect(Token::COMMA, &Token::COMMA.as_expect_error())?;
        }

        struct_fields.insert(field_name, field_type);
    }

    parser.expect(Token::CLOSE_CURLY, &Token::CLOSE_CURLY.as_expect_error())?;
    return Some(return_struct_declaration_statement(struct_name, struct_fields));
}

pub fn parse_variable_declaration_statement(parser: &mut Parser) -> Option<StatementEnum> {
    let is_constant = parser.advance().get_token() == Token::CONST;

    if parser.get_current_token() == Token::OPEN_BRACKET { // if it hits ( then it's a tuple declaration
        return parse_tuple_declaration(parser, is_constant);
    }

    let mut variable_name_token = parser.expect(Token::SYMBOL, "Expects a name for this variable")?;
    let variable_name = get_name_from_symbol(variable_name_token.take_value());

    parser.expect(Token::COLON, &Token::COLON.as_expect_error())?;
    
    let explicit_type = parse_type(parser)?;

    parser.expect(Token::ASSIGN, &Token::ASSIGN.as_expect_error())?;
    
    let value = parse_expression(parser, BindingPower::DefaultBp)?;
    parser.expect(Token::SEMICOLON, &Token::SEMICOLON.as_expect_error())?;

    Some(return_variable_declaration_statement(variable_name, is_constant, value, explicit_type))
}

fn parse_tuple_declaration(parser: &mut Parser, is_constant: bool) -> Option<StatementEnum> {
    parser.expect(Token::OPEN_BRACKET, &Token::OPEN_BRACKET.as_expect_error())?;

    let mut variable_name_and_type: Vec<(String, TypesEnum)> = function_declaration_helper_methods::parse_function_parameters(parser, &format!("Tuple"))?;

    parser.expect(Token::CLOSE_BRACKET, &Token::CLOSE_BRACKET.as_expect_error())?;
    parser.expect(Token::ASSIGN, &Token::ASSIGN.as_expect_error())?;

    let value = parse_expression(parser, BindingPower::DefaultBp)?;
    parser.expect(Token::SEMICOLON, &Token::SEMICOLON.as_expect_error())?;

    // if it has a single variable in it, I'll straight up turn it to a variable declaration
    if variable_name_and_type.len() == 1  {
        let (variable_name, explicit_type) = variable_name_and_type.pop().unwrap();
        return Some(return_variable_declaration_statement(variable_name, is_constant, value, explicit_type));
    } else {
        return Some(return_tuple_declaration_statement(is_constant, variable_name_and_type, value));
    }
}

mod function_declaration_helper_methods {
    use super::*;

    pub fn parse_function_parameters(parser: &mut Parser, name: &String) -> Option<Vec<(String, TypesEnum)>> {
        let mut return_value: Vec<(String, TypesEnum)> = Vec::new();

        while parser.get_current_token() != Token::CLOSE_BRACKET {
            let mut param_name_token = parser.expect(Token::SYMBOL, "Expects a name for this param")?;
            let param_name = get_name_from_symbol(param_name_token.take_value());

            if return_value.iter().any(|(s, _)| s == &param_name) { parser.log_error(&format!("'{}' already has a field with name '{}' at {}", name, param_name, param_name_token.get_position()))?; }
            
            parser.expect(Token::COLON, &Token::COLON.as_expect_error())?;
            let param_type = parse_type(parser)?;

            let is_final_iteration = parser.get_current_token() == Token::CLOSE_BRACKET;
            if !is_final_iteration {
                parser.expect(Token::COMMA, &Token::COMMA.as_expect_error())?;
            }

            return_value.push((param_name, param_type));
        }

        Some(return_value)
    }
}

pub fn parse_function_declaration_statement(parser: &mut Parser) -> Option<StatementEnum> {
    parser.advance(); // it moves past the 'munk'

    let mut function_name_token = parser.expect(Token::SYMBOL, "Expects a name for function")?;
    let function_name = get_name_from_symbol(function_name_token.take_value());

    parser.expect(Token::OPEN_BRACKET, &Token::OPEN_BRACKET.as_expect_error())?;

    let function_parameters: Vec<(String, TypesEnum)> = function_declaration_helper_methods::parse_function_parameters(parser, &format!("Function"))?;

    parser.expect(Token::CLOSE_BRACKET, &Token::CLOSE_BRACKET.as_expect_error())?;
    let function_return_type: TypesEnum;

    if parser.get_current_token() == Token::DASH_GREATER { 
        parser.advance(); // it has a return type
        function_return_type = parse_type(parser)?;
    } else {
        // no return type means an empty tuple as it's return
        function_return_type = return_empty_tuple_type();
    }

    parser.expect(Token::OPEN_CURLY, &Token::OPEN_CURLY.as_expect_error())?;
    let mut function_body = BlockStatement::default();

    while parser.get_current_token() != Token::CLOSE_CURLY {
        function_body.body.push(parse_statement(parser, true)?);
    }

    parser.expect(Token::CLOSE_CURLY, &Token::CLOSE_CURLY.as_expect_error())?;
    Some(return_function_declaration_statement(function_name, function_parameters, function_return_type, function_body))
}