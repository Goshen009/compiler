mod lexer;






const FILEPATH: &str = "./src.txt";

// fn main() {
//     let src_code = get_source_code(FILEPATH);

//     // lexer::find_next_token();
// }

pub fn get_source_code(filepath: &str) -> String {
    let file = std::fs::read_to_string(filepath);
    match file {
        Ok(file_contents) => file_contents,
        Err(error) => {
            println!("Error getting source code. Check if it is the correct file name");
            panic!("{}", error);
        }
    }
}



use logos::{Lexer, Logos, Skip};

#[derive(Debug)]
struct LexerState {
    line: usize,
    column: usize,
}

impl Default for LexerState {
    fn default() -> Self {
        Self {
            line: 1,      // Start from line 1
            column: 0,    // Start from column 1
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(extras = LexerState)]
enum Token {
    #[regex(r"=>", set_line)]
    GreaterEquals,

    #[regex(r"-", set_line)]
    Minus,

    #[regex(r"\r?\n", create_new_line)]
    NewLine,

    #[regex(r"[^\S\r\n]+", set_line)]
    Space,
}

fn create_new_line(lexer: &mut Lexer<Token>) -> Skip {
    lexer.extras.line += 1;
    lexer.extras.column = 1;

    Skip
}

fn set_line(lexer: &mut Lexer<Token>) {
    lexer.extras.column += lexer.span().end - lexer.span().start;
}


fn main() {
    let src_code = get_source_code(FILEPATH);

    lexer::get_tokens(&src_code);


    // lexer::get_tokens("hihi".to_string());

    // let input = "-tyt =>";
    // let mut lexer = Token::lexer(input);

    // // let mut lexer2 = Token::lexer_with_extras(input, extras);

    // // while let Some(token) = lexer.next() {
        
    //     // let span = lexer.span();
    //     // println!("{:?} at {}..{}, {}...{:?}", token, span.start, span.end, lexer.slice(), &lexer.extras);
    // // }

    // while let Some(token) = lexer.next() {
    //     match token {
    //         Err(_) => {
    //             lexer.extras.column += lexer.span().end - lexer.span().start;
    //             // println!("hi");
    //         }
    //         Ok(_) => {

    //         }
    //     }

    //     let span = lexer.span();
    //     println!("{:?} at {}..{}, {}...{:?}", token, span.start, span.end, lexer.slice(), &lexer.extras);

    //     // let state = &lexer.extras;
    //     // match token {
    //     //     Token::Word => println!("Word '{}' at ({}, {})", lexer.slice(), state.line, state.column),
    //     //     Token::Number => println!("Number '{}' at ({}, {})", lexer.slice(), state.line, state.column),
    //     //     Token::Period => println!("Period '.' at ({}, {})", state.line, state.column),
    //     //     Token::Comma => println!("Comma ',' at ({}, {})", state.line, state.column),
    //     //     Token::Error => (), // Errors are already handled in `handle_error`
    //     //     _ => println!("Unhandled token '{}' at ({}, {})", lexer.slice(), state.line, state.column),
    //     // }
    // }

    // add an error callback

    // let input = "hello 123\nworld";
    // let mut lexer = Token::lexer(input);

    // while let Some(token) = lexer.next() {
    //     let span = lexer.span(); // Get start and end positions
        
    // }
}