use super::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
// use std::hash::{Hash, Hasher};

static TYPES : Lazy<HashMap<&'static str, Types>> = Lazy::new(|| {
    HashMap::from([
        ("Number", Types::Number),
        ("String", Types::String)
    ])
});

pub fn parse_type(parser: &mut Parser) -> Option<TypesEnum> {
    let nud_type_fn = parser.get_current_token().nud_type();
    if nud_type_fn.is_none() {
        let error = format!("Expects a type at {}", parser.advance().get_position());
        parser.errors.add_error(error);
        return None;
    }

    return (nud_type_fn.unwrap())(parser);
}

pub fn parse_primary_type(parser: &mut Parser) -> Option<TypesEnum> {
    let mut type_token = parser.advance(); // we're at the SYMBOL
    let type_string = get_name_from_symbol(type_token.take_value());

    let typeq;

    if let Some(type_) = TYPES.get(type_string.as_str()) {
        typeq = type_.clone();
    } else {
        // let mut hasher = std::collections::hash_map::DefaultHasher::new();
        // type_string.hash(&mut hasher);

        // typeq = Types::User_Defined(hasher.finish());

        typeq = Types::User_Defined(type_string);
    }

    Some(return_primary_type(typeq))
}

pub fn parse_array_type(parser: &mut Parser) -> Option<TypesEnum> {
    parser.advance(); // we've gone past the [
    parser.expect(Token::CLOSE_SQUARE, &Token::CLOSE_SQUARE.as_expect_error())?;

    let underlying = parse_type(parser)?;
    Some(return_array_type(underlying))
}

pub fn parse_tuple_type(parser: &mut Parser) -> Option<TypesEnum> {
    parser.advance(); // we've gone past the (

    let mut tuple: Vec<TypesEnum> = Vec::new();

    while parser.get_current_token() != Token::CLOSE_BRACKET {
        let value = parse_type(parser)?;
        tuple.push(value);

        let is_final_iteration = parser.get_current_token() == Token::CLOSE_BRACKET;
        if !is_final_iteration {
            parser.expect(Token::COMMA, &Token::COMMA.as_expect_error())?;
        }
    }

    parser.expect(Token::CLOSE_BRACKET, &Token::CLOSE_BRACKET.as_expect_error())?;

    if tuple.len() == 1 { // set it to be a primary type
        return tuple.pop();
    }

    Some(return_tuple_type(tuple))
}