use super::lexer::{objects::{Position, Lexer}, tokens::Token};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum LexerErrorTypes{ // the 'usize' refers to the index of the token that logged the error.
    Match_Not_Found {
        value: String,
        index: usize,
    },

    Colon_Expected(usize),
    Open_Curly_Expected(usize),
    Colon_Or_Close_Bracket_Expected(usize),

    Struct_Expects_Symbol_Ahead(usize),
    Struct_Expects_End_Statement_Behind(usize),
    Struct_Name_Expects_Open_Curly_In_Front(usize),

    Expected_Type_For_Parameter(usize),
    Function_Expects_Symbol_Ahead(usize),
    Function_Expects_End_Statement_Behind(usize),
    Function_Name_Expects_Open_Bracket_In_Front(usize),
    Function_Expects_Symbol_As_Its_Parameter(usize),

    Opened_Grouping_That_Was_Never_Closed(usize),
    Closed_Grouping_That_Was_Never_Opened(usize),

    Started_String_That_Was_Never_Ended(Position),
}

#[derive(Debug)]
pub struct LexerError {
    pub errors: Vec<LexerErrorTypes>,
    pub open_curly_indexes: Vec<usize>,
    pub open_square_indexes: Vec<usize>,
    pub open_bracket_indexes: Vec<usize>,
}

impl LexerError {
    pub fn new() -> Self {
        Self { 
            errors: Vec::new(),
            open_curly_indexes: Vec::new(),
            open_square_indexes: Vec::new(),
            open_bracket_indexes: Vec::new()
        }
    }

    pub fn add_error(&mut self, error: LexerErrorTypes) {    
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn print_errors(&self, lexer: &Lexer) {
        println!("\n\nERRORS: ");

        self.errors.iter().for_each(|error| {
            match error {
                LexerErrorTypes::Match_Not_Found{ value, index} => {
                    let token = lexer.get_token_at(index);
                    println!("Invalid token '{}' {}", value, token.get_position());
                }

                LexerErrorTypes::Colon_Expected(index) => {
                    let token = lexer.get_token_at(index);
                    println!("Expected ':' {}", token.get_position());
                }
                LexerErrorTypes::Open_Curly_Expected(index) => {
                    let token = lexer.get_token_at(index);
                    println!("Expected '{{' {}", token.get_position());
                }
                LexerErrorTypes::Colon_Or_Close_Bracket_Expected(index) => {
                    let token = lexer.get_token_at(index);
                    println!("Expected ')' or ',' {}", token.get_position());
                }

                LexerErrorTypes::Struct_Expects_Symbol_Ahead(index) => {
                    let token = lexer.get_token_at(index);
                    println!("Expected a symbol ahead of keyword STRUCT {}", token.get_position());
                }
                LexerErrorTypes::Struct_Expects_End_Statement_Behind(index) => {
                    let token = lexer.get_token_at(index);
                    println!("Expected a ';' or '}}' behind keyword STRUCT {}", token.get_position());
                }
                LexerErrorTypes::Struct_Name_Expects_Open_Curly_In_Front(index) => {
                    let token = lexer.get_token_at(index);
                    let token_string = token.get_value().as_string().unwrap();
                    println!("Expected a '{{' ahead of {} {}", token_string, token.get_position());
                }

                LexerErrorTypes::Expected_Type_For_Parameter(index) => {
                    let token = lexer.get_token_at(index);
                    println!("Expected a TYPE for parameter {}", token.get_position());
                }
                LexerErrorTypes::Function_Expects_Symbol_Ahead(index) => {
                    let token = lexer.get_token_at(index);
                    println!("Expected a symbol ahead of keyword FUNC {}", token.get_position());
                }
                LexerErrorTypes::Function_Expects_End_Statement_Behind(index) => {
                    let token = lexer.get_token_at(index);
                    println!("Expected a ';' or '}}' behind keyword FUNC {}", token.get_position());
                }
                LexerErrorTypes::Function_Name_Expects_Open_Bracket_In_Front(index) => {
                    let token = lexer.get_token_at(index);
                    let token_string = token.get_value().as_string().unwrap();
                    println!("Expected a '(' ahead of function name '{}' {}", token_string, token.get_position());
                }
                LexerErrorTypes::Function_Expects_Symbol_As_Its_Parameter(index) => {
                    let token = lexer.get_token_at(index);
                    println!("Function parameter should be a SYMBOL {}", token.get_position());
                }

                LexerErrorTypes::Opened_Grouping_That_Was_Never_Closed(index) => {
                    let token = lexer.get_token_at(index);
                    println!("'{}' {} was never closed", token.get_token(), token.get_position());
                }
                LexerErrorTypes::Closed_Grouping_That_Was_Never_Opened(index) => {
                    let token = lexer.get_token_at(index);
                    println!("'{}' {} was never opened", token.get_token(), token.get_position());
                }

                LexerErrorTypes::Started_String_That_Was_Never_Ended(position) => {
                    println!("You started a string {} but did not close it", position);
                }
            }
        });
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ParserErrorTypes{
    Expected_Token {
        expected_token: Token,
        found_token: Token,
        position: Position
    },

    Random,
}

#[derive(Debug)]
pub struct ParserError {
    pub errors: Vec<ParserErrorTypes>
}

impl ParserError {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: ParserErrorTypes) {
        self.errors.push(error);
    }
}