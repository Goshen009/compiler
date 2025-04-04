use super::*;

// It had no support for multiple statements in one i.e Tuple
// So I had it return a Vec<StatementEnum> to handle that.
// UPDATE: I have support now, but I'm still keeping it that way.
pub fn parse_statement(parser: &mut Parser) -> Vec<StatementEnum> {
    let stmt_fn = parser.get_current_token().stmt();
    if let Some(func) = stmt_fn {
        return func(parser);
    }

    let expression = parse_expression(parser, BindingPower::DefaultBp);
    parser.expect(Token::SEMICOLON);

    vec![return_expression_statement(expression)]
}

fn parse_tuple_declaration(parser: &mut Parser, is_constant: bool) -> Vec<StatementEnum> {
    // we start at open bracket!
    parser.expect(Token::OPEN_BRACKET);

    let mut variable_name_and_type: Vec<(String, TypesEnum)> = function_declaration_helper_methods::parse_function_parameters(parser);

    parser.expect(Token::CLOSE_BRACKET);
    parser.expect(Token::ASSIGN);

    let value = parse_expression(parser, BindingPower::DefaultBp);
    parser.expect(Token::SEMICOLON);

    // If it has a single variable in it, I'll straight up turn it to a variable declaration
    if variable_name_and_type.len() == 1  {
        let (variable_name, explicit_type) = variable_name_and_type.pop().unwrap();
        return vec![return_variable_declaration_statement(variable_name, is_constant, value, explicit_type)];
    } else {
        return vec![return_tuple_declaration_statement(is_constant, variable_name_and_type, value)];
    }
}

pub fn parse_variable_declaration_statement(parser: &mut Parser) -> Vec<StatementEnum> {
    let is_constant = parser.advance().get_token() == Token::CONST;

    if parser.get_current_token() == Token::OPEN_BRACKET {
        // if it hits an open bracket then we know it's a tuple being declared.
        return parse_tuple_declaration(parser, is_constant);
    }

    let current_token = parser.expect_error(Token::SYMBOL, "Expected a variable name in this declaration! Come on dummy!");
    let variable_name = get_name_from_symbol(current_token.get_token_value());

    parser.expect(Token::COLON);
    let explicit_type = parse_type(parser);

    parser.expect(Token::ASSIGN);

    let value = parse_expression(parser, BindingPower::DefaultBp);
    parser.expect(Token::SEMICOLON);

    vec![return_variable_declaration_statement(variable_name, is_constant, value, explicit_type)]
}

pub fn parse_struct_declaration_statement(parser: &mut Parser) -> Vec<StatementEnum> {
    parser.advance(); // it moves past the 'struct'

    let current_token = parser.expect_error(Token::SYMBOL, "Name your structs properly will ya. Do better!");
    let struct_name = get_name_from_symbol(current_token.get_token_value());

    parser.expect(Token::OPEN_CURLY);
    let mut struct_fields: HashMap<String, TypesEnum> = HashMap::new();

    while parser.get_current_token() != Token::CLOSE_CURLY {
        let current_token = parser.expect_error(Token::SYMBOL, "I'm expecting a name for your fields. *sigh* ");
        let field_name = get_name_from_symbol(current_token.get_token_value());

        // does a check to see if the hashmap already has that value in it.
        if struct_fields.contains_key(&field_name) { panic!("Do I have to keep insulting you? Use a name that hasn't been used before!!"); }

        parser.expect_error(Token::COLON, "Now you've gone and forgot a : or were you trying not to give it a type?!");
        let field_type = parse_type(parser);

        let is_final_iteration = parser.get_current_token() == Token::CLOSE_CURLY;
        if !is_final_iteration {
            parser.expect_error(Token::COMMA, "FOOL! You forgot the comma at the end!");
        }
        
        struct_fields.insert(field_name, field_type);
    }

    parser.expect_error(Token::CLOSE_CURLY, "Now won't you like to close your structs? Some programmer you are.");
    vec![return_struct_declaration_statement(struct_name, struct_fields)]
}

mod function_declaration_helper_methods{
    use super::*;

    pub fn parse_function_parameters(parser: &mut Parser) -> Vec<(String, TypesEnum)> {
        let mut return_value: Vec<(String, TypesEnum)> = Vec::new();

        while parser.get_current_token() != Token::CLOSE_BRACKET {
            let current_token = parser.expect_error(Token::SYMBOL, "Oh really! Now you won't give the parameters a name?");
            let param_name = get_name_from_symbol(current_token.get_token_value());

            // checks if the vector contains the same name
            if return_value.iter().any(|(s, _)| s == &param_name) { panic!("Do I have to keep insulting you? Use a name that hasn't been used before!!"); }

            parser.expect_error(Token::COLON, "Alright then. Don't give your parameters a type. Mvchewwwww");
            let param_type = parse_type(parser);

            expect_comma(parser);

            return_value.push((param_name, param_type));
        }

        return_value
    }
}

pub fn parse_function_declaration_statement(parser: &mut Parser) -> Vec<StatementEnum> {
    parser.advance(); // it moves past the 'munk'

    let current_token = parser.expect_error(Token::SYMBOL, "Name your function abeg");
    let function_name = get_name_from_symbol(current_token.get_token_value());

    parser.expect(Token::OPEN_BRACKET);
    let function_parameters: Vec<(String, TypesEnum)> = function_declaration_helper_methods::parse_function_parameters(parser);

    parser.expect(Token::CLOSE_BRACKET);
    let function_return_type: TypesEnum;

    if parser.get_current_token() == Token::DASH_GREATER { 
        // it has a return type
        parser.advance();
        function_return_type = parse_type(parser);
    } else {
        // no return type means an empty tuple as it's return
        function_return_type = return_empty_tuple_type();
    }

    parser.expect(Token::OPEN_CURLY);
    let mut function_body = BlockStatement {
        body: Vec::new()
    };

    while parser.get_current_token() != Token::CLOSE_CURLY {
        // function_body.body.push(parse_statement(parser))
        for statement in parse_statement(parser) {
            function_body.body.push(statement);
        }
    }

    parser.expect(Token::CLOSE_CURLY);    
    vec![return_function_declaration_statement(function_name, function_parameters, function_return_type, function_body)]
}