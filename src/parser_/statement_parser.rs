










use super::Parser;
use crate::StatementEnum;

pub type StmtHandler = fn(&mut Parser) -> StatementEnum;

// pub fn parse_statement(parser: &mut Parser) -> Result<StatementEnum, ()> {
//     let stmt_fn = parser.get_current_token().stmt();
//     if stmt_fn.is_err() {
//         // call the function that tries to salvage what's left and skip to the next token for parsing
//         return Err(());
//     }

//     let statement = stmt_fn.unwrap()(parser);
//     return Ok(statement);
// }


// pub fn parse_statement(parser: &mut Parser) -> Vec<StatementEnum> {
//     let stmt_fn = parser.get_current_token().stmt();
//     if let Some(func) = stmt_fn {
//         return func(parser);
//     }

//     let expression = parse_expression(parser, BindingPower::DefaultBp);
//     parser.expect(Token::SEMICOLON);

//     vec![return_expression_statement(expression)]
// }






// use super::ParserErrorTypes;
// use super::Token;


// struct ade {
//     : i128,
//     field2: i16
// }


// pub fn parse_struct_declaration_statement(parser: &mut Parser) -> Result<StatementEnum, ()> {
//     parser.expect(Token::STRUCT); // we start at 'struct' and we don't check the result because it'll always be true.

//     let next_token = parser.expect(Token::SYMBOL);
//     if next_token.is_none() {
//         // found_error = true;
//     }

//     if parser.expect(Token::OPEN_CURLY).is_none() {
//         found_error = Token::OPEN_CURLY;
//     }
// }



// pub fn parse_struct_declaration_statement(parser: &mut Parser) -> Vec<StatementEnum> {
//     parser.advance(); // it moves past the 'struct'

//     let current_token = parser.expect_error(Token::SYMBOL, "Name your structs properly will ya. Do better!");
//     let struct_name = get_name_from_symbol(current_token.get_token_value());

//     parser.expect(Token::OPEN_CURLY);
//     let mut struct_fields: HashMap<String, TypesEnum> = HashMap::new();

//     while parser.get_current_token() != Token::CLOSE_CURLY {
//         let current_token = parser.expect_error(Token::SYMBOL, "I'm expecting a name for your fields. *sigh* ");
//         let field_name = get_name_from_symbol(current_token.get_token_value());

//         // does a check to see if the hashmap already has that value in it.
//         if struct_fields.contains_key(&field_name) { panic!("Do I have to keep insulting you? Use a name that hasn't been used before!!"); }

//         parser.expect_error(Token::COLON, "Now you've gone and forgot a : or were you trying not to give it a type?!");
//         let field_type = parse_type(parser);

//         let is_final_iteration = parser.get_current_token() == Token::CLOSE_CURLY;
//         if !is_final_iteration {
//             parser.expect_error(Token::COMMA, "FOOL! You forgot the comma at the end!");
//         }
        
//         struct_fields.insert(field_name, field_type);
//     }

//     parser.expect_error(Token::CLOSE_CURLY, "Now won't you like to close your structs? Some programmer you are.");
//     vec![return_struct_declaration_statement(struct_name, struct_fields)]
// }