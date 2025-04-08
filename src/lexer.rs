mod tokens;
mod objects;

use objects::*;
use tokens::Token;

use logos::Logos;

pub fn get_tokens(src_code: &str) -> LexerObject {
    let lexer_object = LexerObject::default();

    let mut lexer = Token::lexer(src_code);

    while let Some(value) = lexer.next() {
        // match value {
        //     Ok(token) => {
                
        //     },

        //     Err(_) => {

        //     }
        // }

        let span = lexer.span();
        println!("{:?} at {}..{}, {:?}, Value: '{}'", value, span.start, span.end, &lexer.extras.position, &lexer.extras.value);
    }

    return lexer_object;
}


fn main() {
    let input = "-tyt =>";
    let mut lexer = Token::lexer(input);

    // let mut lexer2 = Token::lexer_with_extras(input, extras);

    // while let Some(token) = lexer.next() {
        
    //     let span = lexer.span();
    //     println!("{:?} at {}..{}, {}...{:?}", token, span.start, span.end, lexer.slice(), &lexer.extras);
    // }

    while let Some(token) = lexer.next() {
        match token {
            Err(_) => {
                // lexer.extras.column += lexer.span().end - lexer.span().start;
                // println!("hi");
            }
            Ok(_) => {

            }
        }

        let span = lexer.span();
        // println!("{:?} at {}..{}, {}...{:?}", token, span.start, span.end, lexer.slice(), &lexer.extras);

        // let state = &lexer.extras;
        // match token {
        //     Token::Word => println!("Word '{}' at ({}, {})", lexer.slice(), state.line, state.column),
        //     Token::Number => println!("Number '{}' at ({}, {})", lexer.slice(), state.line, state.column),
        //     Token::Period => println!("Period '.' at ({}, {})", state.line, state.column),
        //     Token::Comma => println!("Comma ',' at ({}, {})", state.line, state.column),
        //     Token::Error => (), // Errors are already handled in `handle_error`
        //     _ => println!("Unhandled token '{}' at ({}, {})", lexer.slice(), state.line, state.column),
        // }
    }

    // add an error callback

    // let input = "hello 123\nworld";
    // let mut lexer = Token::lexer(input);

    // while let Some(token) = lexer.next() {
    //     let span = lexer.span(); // Get start and end positions
        
    // }
}