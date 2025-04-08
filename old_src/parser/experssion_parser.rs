use super::{*, handlers::BindingPower};
use std::collections::HashMap;

pub fn parse_expression(parser: &mut Parser, bp: BindingPower) -> Option<ExpressionEnum> {
    let nud_fn = parser.get_current_token().nud();
    if nud_fn.is_none() {
        let error = format!("Expects a nud_fn at {}", parser.advance().get_position());
        parser.errors.add_error(error);
        return None;
    }

    let mut left = (nud_fn.unwrap())(parser)?;

    while parser.get_current_token().binding_power() > bp {
        let led_fn = parser.get_current_token().led();
        if led_fn.is_none() {
            let error = format!("Expects a led_fn at {}", parser.advance().get_position());
            parser.errors.add_error(error);
            return None;
        }

        left = (led_fn.unwrap())(parser, left, parser.get_current_token().binding_power())?;
    }

    Some(left)
}

pub fn parse_struct_initialization_expression(parser: &mut Parser, struct_name: String) -> Option<ExpressionEnum> {
    parser.advance(); // moves past the '{'
    let mut struct_fields: HashMap<String, ExpressionEnum> = HashMap::new();

    while parser.get_current_token() != Token::CLOSE_CURLY {
        let mut field_name_token = parser.expect(Token::SYMBOL, &format!("Expects a field for struct '{}'", struct_name))?;
        let field_name = get_name_from_symbol(field_name_token.take_value());

        // checks if the hashmap already has a field with that name
        if struct_fields.contains_key(&field_name) { parser.log_error(&format!("Struct '{}' already has a field with name '{}' at {}", struct_name, field_name, field_name_token.get_position()))?; }

        parser.expect(Token::COLON, &Token::COLON.as_expect_error())?;
        let field_value = parse_expression(parser, BindingPower::DefaultBp)?;

        // for trailing commas
        let is_final_iteration = parser.get_current_token() == Token::CLOSE_CURLY;
        if !is_final_iteration {
            parser.expect(Token::COMMA, &Token::COMMA.as_expect_error())?;
        }
        
        struct_fields.insert(field_name, field_value);
    }

    parser.expect(Token::CLOSE_CURLY, &Token::CLOSE_CURLY.as_expect_error())?;
    Some(return_struct_assignment_expression(struct_name, struct_fields))
}

// pub fn parse_assignment_expression(parser: &mut Parser, left: ExpressionEnum, bp: BindingPower) -> Option<ExpressionEnum> {
//     println!("the left was: {:?}", left);

//     let operator_kind = parser.advance().get_token();
//     let right = parse_expression(parser, bp)?;

//     Some(return_assignment_expression(left, operator_kind, right))
// }

pub fn parse_binary_expression(parser: &mut Parser, left: ExpressionEnum, bp: BindingPower) -> Option<ExpressionEnum> {   
    let operator = parser.advance().get_token();
    let right = parse_expression(parser, bp)?;
    
    Some(return_binary_expression(left, operator, right))
}

fn parse_function_call_expression(parser: &mut Parser, function_name: String) -> Option<ExpressionEnum> {
    // we start at the open brackets
    let function_argument = parse_expression(parser, BindingPower::DefaultBp)?;
    
    Some(return_function_call_expression(function_name, function_argument))
}

fn parse_tuple_expression(parser: &mut Parser, left: ExpressionEnum) -> Option<ExpressionEnum> {
    parser.advance(); // we start at the comma
    let mut tuple: Vec<ExpressionEnum> = Vec::from([left]);

    while parser.get_current_token() != Token::CLOSE_BRACKET {
        let value = parse_expression(parser, BindingPower::DefaultBp)?;
        tuple.push(value);

        let is_final_iteration = parser.get_current_token() == Token::CLOSE_BRACKET;
        if !is_final_iteration {
            parser.expect(Token::COMMA, &Token::COMMA.as_expect_error())?;
        }
    }

    Some(return_tuple_assignment_expression(tuple))
}

pub fn parse_bracket_expression(parser: &mut Parser) -> Option<ExpressionEnum> {
    parser.advance(); // we start at open bracket

    if parser.get_current_token() == Token::CLOSE_BRACKET {
        // it's an empty bracket -- treat as empty tuple
        parser.advance();
        return Some(return_empty_tuple());
    }

    let mut value = parse_expression(parser, BindingPower::DefaultBp)?;
    
    if parser.get_current_token() == Token::COMMA { // do not change this to expect_comma();
        value = parse_tuple_expression(parser, value)?;
    }
    
    parser.expect(Token::CLOSE_BRACKET, &Token::CLOSE_BRACKET.as_expect_error())?;
    Some(value)
}

pub fn parse_array_expression(parser: &mut Parser) -> Option<ExpressionEnum>{
    parser.advance(); // we begin at the [
    let mut array_value: Vec<ExpressionEnum> = Vec::new();

    while parser.get_current_token() != Token::CLOSE_SQUARE {
        array_value.push(parse_expression(parser, BindingPower::DefaultBp)?);

        let is_final_iteration = parser.get_current_token() == Token::CLOSE_SQUARE;
        if !is_final_iteration {
            parser.expect(Token::COMMA, &Token::COMMA.as_expect_error())?;
        }
    }

    parser.expect(Token::CLOSE_SQUARE, &Token::CLOSE_SQUARE.as_expect_error())?;
    Some(return_array_expression(array_value))
}

pub fn parse_prefix_expression(parser: &mut Parser) -> Option<ExpressionEnum> {
    let sign = parser.advance().get_token();

    let right_hand = parse_expression(parser, BindingPower::DefaultBp)?;
    Some(return_prefix_expression(sign, right_hand))
}

pub fn parse_primary_expression(parser: &mut Parser) -> Option<ExpressionEnum> {
    let mut current_token = parser.advance();
    let current_token_type = current_token.get_token();

    let mut num_value = 0;
    let mut string_value = String::new();
    match current_token.take_value() {
        TokenValue::Number(val) => num_value = val,
        TokenValue::String(val) => string_value = val,
        TokenValue::None => (),
    }
    
    match current_token_type {
        Token::NUMBER => Some(return_number_expression(num_value)),
        Token::STRING => Some(return_string_expression(string_value)),
        Token::SYMBOL => {
            if parser.get_current_token() == Token::OPEN_BRACKET {
                // It is a function call. 
                // Function calls are symbols followed by an open bracket
                parse_function_call_expression(parser, string_value)
            } 
            else if parser.get_current_token() == Token::OPEN_CURLY {
                // It's a struct initialization.
                parse_struct_initialization_expression(parser, string_value)
            }
            else {
                Some(return_symbol_expression(string_value))
            }
        },
        _ => panic!("An error occured trying to parse a primary expression. This should never happen though. It isn't a string, symbol or number"),
    }
}