mod lexer;
// pub mod parser;
pub mod ast_types;
// pub mod semantics;
pub mod errorq;
pub mod error_codes;


use lexer::*;
// pub use parser::*;
pub use ast_types::*;
// pub use semantics::*;
pub use errorq::*;
pub use error_codes::*;

const FILEPATH: &str = "./source_code/src2.txt";

pub mod parser_;
use parser_::*;

fn main() {
    let src = get_source_code(FILEPATH);

    lex(src); // it calls into the parser.


    println!("\n\nDO FOR LET! DO FOR TUPLE! FINISH FOR STRUCT!");
    println!("The syntax checking I mean!");
}

pub fn get_source_code(filepath: &str) -> std::iter::Peekable<std::vec::IntoIter<String>> {
    // we'll read the source code line by line as opposed to reading the full thing at once.
    // why? I want to eliminate copying strings in the lexer. And checking for regex line by line will be much quicker when a match is not found as opposed to checking full file for a match.

    use std::io::BufRead;
    
    let file = std::fs::File::open(filepath);
    if file.is_err() {
        panic!("Error reading source code! Please check the file path.");
    } else {
        let reader = std::io::BufReader::new(file.unwrap());
        let line: Vec<String> = reader.lines().filter_map(|line| {
            match line {
                Ok(valid_line) => Some(valid_line),
                Err(e) => panic!("Error turning source code into lines: {e}")
            }
        }).collect();

        // returns an iterator over the lines.
        return line.into_iter().peekable();
    }
}

// fn main() {

//     // let b = ();

//     // let () = ();

//     let src = get_source_code(FILEPATH);

//     let (tokens, lexer_errors) = lex(src);
//     if lexer_errors.errors.len() != 0 {
//         lexer_errors.print_errors();
//         return;
//     }

//     let program = parse(tokens);

//     // start_semantics(program);


//     // // and checking on assignments
// }














// pub fn get_source_code(filepath: &str) -> String {
//     let file = std::fs::read_to_string(filepath);
//     let contents = match file {
//         Ok(file_contents) => file_contents,
//         Err(error) => {
//             println!("Error: {}", error);
//             println!("Defaulting to boilerplate code");
//             String::from("10 * 5 + 2")
//         }
//     };

//     contents
// }


// #[cfg(test)]
// mod tests {
//     use std::{collections::HashMap, vec};
//     use super::*;

//     #[test]
//     fn test_variable_declaration() {
//         let variable_program = Program {
//             code: vec![
//                 return_variable_declaration_statement("a".to_string(), false, return_number_expression(
//                     90
//                 ), return_primary_type(
//                     Types::Number
//                 )),

//                 return_variable_declaration_statement("b".to_string(), false, return_string_expression(
//                     "Hallo".to_string()
//                 ), return_primary_type(
//                     Types::String
//                 )),

//                 return_variable_declaration_statement("c".to_string(), true, return_array_expression(
//                     vec![
//                         return_symbol_expression("XYZ".to_string()),
//                         return_symbol_expression("OOP".to_string())
//                     ]
//                 ), return_array_type(
//                     return_primary_type(
//                         Types::User_Defined(10048736038501913812)
//                     )
//                 )),

//                 return_variable_declaration_statement("f".to_string(), false, return_symbol_expression(
//                     "POP".to_string()
//                 ), return_primary_type(
//                     Types::User_Defined(10048736038501913812)
//                 )),

//                 return_variable_declaration_statement("e".to_string(), false, return_string_expression(
//                     "Welcome".to_string()
//                 ), return_primary_type(
//                     Types::String
//                 )),

//                 return_variable_declaration_statement("d".to_string(), false, return_number_expression(
//                     90
//                 ), return_primary_type(
//                     Types::Number
//                 ))
//             ]
//         };

//         const FILEPATH: &str = "./source_code/test_variable_declaration.txt";
//         let src = get_source_code(FILEPATH);
//         let tokens = lex(src);
//         let program = parse(tokens);

//         assert_eq!(program, variable_program, "The variable ast has an issue");
//     }

//     #[test]
//     fn test_struct_declaration() {
//         let struct_program = Program {
//             code: vec![
//                 return_struct_declaration_statement("TestStruct".to_string(), HashMap::from([
//                     ("field1".to_string(), return_primary_type(
//                         Types::Number
//                     )),
//                     ("field2".to_string(), return_primary_type(
//                         Types::User_Defined(10048736038501913812)
//                     )),
//                     ("field3".to_string(), return_array_type(
//                         return_primary_type(Types::String)
//                     ))
//                 ]))
//             ]
//         };

//         const FILEPATH: &str = "./source_code/test_struct_declaration.txt";
//         let src = get_source_code(FILEPATH);
//         let tokens = lex(src);
//         let program = parse(tokens);

//         assert_eq!(program, struct_program, "The struct ast has an issue");
//     }

//     #[test]
//     fn test_function_declaration() {
//         let func_program = Program {
//             code: vec![
//                 return_function_declaration_statement(String::from("main"), Vec::new(), return_tuple_type(Vec::new()), BlockStatement {
//                     body: vec![]
//                 }),

//                 return_function_declaration_statement(String::from("main_with_parameters"), vec![
//                     ("a".to_string(), return_primary_type(
//                         Types::Number
//                     )),
//                     ("b".to_string(), return_primary_type(
//                         Types::String
//                     ))
//                 ], return_tuple_type(Vec::new()), BlockStatement {
//                     body: vec![]
//                 }),

//                 return_function_declaration_statement(String::from("main_with_return_type"), vec![
//                     ("a".to_string(), return_primary_type(
//                         Types::Number
//                     )),
//                     ("b".to_string(), return_primary_type(
//                         Types::User_Defined(10048736038501913812)
//                     ))
//                 ], return_tuple_type( vec![
//                     TypesEnum::Primary({
//                         PrimaryType { type_: Types::String }
//                     })
//                 ]), BlockStatement {
//                     body: vec![]
//                 }),

//                 return_function_declaration_statement(String::from("main_return_no_param"), Vec::new()
//                 , return_tuple_type( vec![
//                     TypesEnum::Primary({
//                         PrimaryType { type_: Types::String }
//                     })
//                 ]), BlockStatement {
//                     body: vec![]
//                 }),
//             ]
//         };

//         const FILEPATH: &str = "./source_code/test_function_declaration.txt";
//         let src = get_source_code(FILEPATH);
//         let tokens = lex(src);
//         let program = parse(tokens);

//         assert_eq!(program, func_program, "The function ast has an issue");
//     }
// }