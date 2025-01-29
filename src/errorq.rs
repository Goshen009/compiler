use super::lexer::{objects::Position, tokens::Token};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum LexerErrorTypes<'a>{ // the 'usize' refers to the index of the token that logged the error.
    Expects_Type,
    Expects_Name,

    Expects_Token(&'a str),
    Expects_Two_Tokens(&'a str, &'a str),

    Unexpected_Token(&'a str),
}

pub fn get_error_string(err_type: LexerErrorTypes, position: Position) -> String {
    match err_type {
        LexerErrorTypes::Expects_Token(token) => format!("Expected '{token}' at {position}"),
        LexerErrorTypes::Expects_Two_Tokens(token1, token2) => format!("Expected '{token1}' or '{token2}' at {position}"),

        LexerErrorTypes::Unexpected_Token(token) => format!("Unexpected token '{token}' at {position}"),

        LexerErrorTypes::Expects_Name => format!("Expected a name at {position}"),
        LexerErrorTypes::Expects_Type => format!("Expected a type at {position}"),
    }
}

#[derive(Debug)]
pub struct LexerError {
    pub errors: Vec<String>,    
    pub open_curly_indexes: Vec<Token>,
    pub open_square_indexes: Vec<Token>,
    pub open_bracket_indexes: Vec<Token>,
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

    pub fn add_error(&mut self, error: String) {    
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn print_errors(&self) {
        println!("\n\nERRORS: ");

        for error in self.errors.iter() {
            println!("{error}");
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ParserErrorTypes<'a>{
    Expects_Name,

    Expects_Token(&'a str),

    Expects_Stmt_Fn,
}

impl <'a>ParserErrorTypes<'a> {
    fn as_string(&self, position: Position) -> String {
        match *self {
            ParserErrorTypes::Expects_Name => format!("Expected a name at {position}"),
            ParserErrorTypes::Expects_Token(token) => format!("Expected '{token}' at {position}"),
            ParserErrorTypes::Expects_Stmt_Fn => format!("Expected a stmt fn at {position}"),
        }
    }
}

#[derive(Debug)]
pub struct ParserError {
    pub errors: Vec<String>
}

impl ParserError {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: ParserErrorTypes, position: Position) {
        self.errors.push(error.as_string(position));
    }

    pub fn print_errors(&self) {
        println!("\n\nERRORS: ");

        for error in self.errors.iter() {
            println!("{error}");
        }
    }
}