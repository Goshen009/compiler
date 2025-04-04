use super::*;

pub fn parse_expression(parser: &mut Parser, bp: BindingPower) -> ExpressionEnum {
    let nud_fn = parser.get_current_token().nud();
    if nud_fn == None {
        panic!("Expected a nud_fn");
    }

    let mut left = nud_fn.unwrap()(parser);

    while parser.get_current_token().binding_power() > bp {
        let led_fn = parser.get_current_token().led();
        if led_fn == None {
            panic!("There's an error in parsing the led_fn");
        }

        left = led_fn.unwrap()(parser, left, parser.get_current_token().binding_power());
    }

    left
}

pub fn parse_struct_initialization_expression(parser: &mut Parser, struct_name: String) -> ExpressionEnum {
    parser.advance(); // moves past the '{'
    let mut struct_fields: HashMap<String, ExpressionEnum> = HashMap::new();

    while parser.get_current_token() != Token::CLOSE_CURLY {
        let current_token = parser.expect_error(Token::SYMBOL, "I'm expecting a name for your fields in the struct initialization. *sigh* ");
        let field_name = get_name_from_symbol(current_token.get_token_value());

        // does a check to see if the hashmap already has that value in it.
        if struct_fields.contains_key(&field_name) { panic!("Do I have to keep insulting you? Use a name that hasn't been used before!!"); }

        parser.expect_error(Token::COLON, "Assign a value to your fields. PLEASE!");
        let field_value = parse_expression(parser, BindingPower::DefaultBp);
        
        struct_fields.insert(field_name, field_value);

        // for trailing commas
        let is_final_iteration = parser.get_current_token() == Token::CLOSE_CURLY;
        if !is_final_iteration {
            parser.expect_error(Token::COMMA, "FOOL! You forgot the comma at the end!");
        }
    }

    parser.expect_error(Token::CLOSE_CURLY, "Now won't you like to close your structs? Some programmer you are.");
    return_struct_assignment_expression(struct_name, struct_fields)
}

pub fn parse_assignment_expression(parser: &mut Parser, left: ExpressionEnum, bp: BindingPower) -> ExpressionEnum {
    println!("the left was: {:?}", left);

    let operator_kind = parser.advance().get_token();
    let right = parse_expression(parser, bp);

    return_assignment_expression(left, operator_kind, right)
}

pub fn parse_binary_expression(parser: &mut Parser, left: ExpressionEnum, bp: BindingPower) -> ExpressionEnum {   
    let operator = parser.advance().get_token();
    let right = parse_expression(parser, bp);
    
    return_binary_expression(left, operator, right)
}

pub fn parse_function_call_expression(parser: &mut Parser, function_name: String) -> ExpressionEnum {
    // we start at the open brackets
    let function_argument = parse_expression(parser, BindingPower::DefaultBp);
    
    return_function_call_expression(function_name, function_argument)
}

fn parse_tuple_expression(parser: &mut Parser, left: ExpressionEnum) -> ExpressionEnum {
    // we start at the comma
    parser.expect(Token::COMMA);
    let mut tuple: Vec<ExpressionEnum> = Vec::from([left]);

    while parser.get_current_token() != Token::CLOSE_BRACKET {
        let value = parse_expression(parser, BindingPower::DefaultBp);
        tuple.push(value);

        expect_comma(parser);
    }

    return_tuple_assignment_expression(tuple)
}

pub fn parse_bracket_expression(parser: &mut Parser) -> ExpressionEnum {
    // we start at open bracket
    parser.expect(Token::OPEN_BRACKET);

    if parser.get_current_token() == Token::CLOSE_BRACKET {
        // it's an empty bracket -- treat as empty tuple
        parser.advance();
        return return_empty_tuple();
    }

    let mut value = parse_expression(parser, BindingPower::DefaultBp);
    
    if parser.get_current_token() == Token::COMMA { // do not change this to expect_comma();
        value = parse_tuple_expression(parser, value);
    }
    
    parser.expect(Token::CLOSE_BRACKET);
    value
}

pub fn parse_array_expression(parser: &mut Parser) -> ExpressionEnum{
    parser.expect(Token::OPEN_SQUARE); // we begin at the [
    let mut array_value: Vec<ExpressionEnum> = Vec::new();

    while parser.get_current_token() != Token::CLOSE_SQUARE {
        array_value.push(parse_expression(parser, BindingPower::DefaultBp));
        
        let is_final_iteration = parser.get_current_token() == Token::CLOSE_SQUARE;
        if !is_final_iteration {
            parser.expect_error(Token::COMMA, "FOOL! You forgot the comma at the end!");
        }
    }

    parser.expect(Token::CLOSE_SQUARE);
    return_array_expression(array_value)
}

pub fn parse_prefix_expression(parser: &mut Parser) -> ExpressionEnum {
    let sign = parser.advance().get_token();

    let nud_fn = parser.get_current_token().nud();
    if nud_fn == None {
        panic!("Expected a nud_fn in the parse_prefix_expression");
    }

    let right_hand = nud_fn.unwrap()(parser);
    return_prefix_expression(sign, right_hand)
}

pub fn parse_primary_expression(parser: &mut Parser) -> ExpressionEnum {
    let current_token = parser.advance();
    let current_token_type = current_token.get_token();

    let mut num_value = 0;
    let mut string_value = String::new();
    match current_token.get_token_value() {
        TokenValue::Number(val) => num_value = val,
        TokenValue::String(val) => string_value = val,
        TokenValue::None => (),
    }
    
    match current_token_type {
        Token::NUMBER => return_number_expression(num_value),
        Token::STRING => return_string_expression(string_value),
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
                return_symbol_expression(string_value)
            }
        },
        _ => panic!("An error occured trying to parse a primary expression. It isn't a string, symbol or number"),
    }
}