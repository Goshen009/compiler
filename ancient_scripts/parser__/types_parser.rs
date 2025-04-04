use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_type_enum(type_string: &String, hash: u64) -> Types {
    let map = HashMap::from([
       (String::from("Number"), Types::Number),
       (String::from("String"), Types::String),
    ]);

    if let Some(val) = map.get(type_string) {
        return val.clone();
    } else {
        return Types::User_Defined(hash);
    }
}

pub fn parse_primary_type(parser: &mut Parser) -> TypesEnum {
    let type_token = parser.advance(); // we're at TYPE
    let type_string = get_name_from_symbol(type_token.get_token_value());

    let mut hasher = DefaultHasher::new();
    type_string.hash(&mut hasher);

    let type_ = get_type_enum(&type_string, hasher.finish());
    return_primary_type(type_)
}

pub fn parse_array_type(parser: &mut Parser) -> TypesEnum {
    parser.advance(); // we've gone past the [
    parser.expect(Token::CLOSE_SQUARE); // checks if the next is ]

    let underlying = parse_type(parser);
    return_array_type(underlying)
}

pub fn parse_tuple_type(parser: &mut Parser) -> TypesEnum {
    parser.advance(); // we've gone past the (

    let mut tuple: Vec<TypesEnum> = Vec::new();

    while parser.get_current_token() != Token::CLOSE_BRACKET {
        let value = parse_type(parser);
        tuple.push(value);

        expect_comma(parser);
    }
    parser.expect(Token::CLOSE_BRACKET);

    if tuple.len() == 1 { // set it to be a primary type.
        return tuple.pop().unwrap();
    }

    return return_tuple_type(tuple)
}

pub fn parse_type(parser: &mut Parser) -> TypesEnum {
    // let var_name: TYPE = 0;
    // let var_name: []TYPE = 0;
    // let var_name: (TYPE, TYPE) = (0, 0);
    
    // we start immediately after the colon
    let nud_type_fn = parser.get_current_token().nud_type();
    if nud_type_fn == None {
        panic!("Expected a nud_type_fn");
    }

    let return_type = nud_type_fn.unwrap()(parser);
    return_type
}