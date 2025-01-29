






use std::collections::{HashMap, VecDeque};
use super::tokens::{TokenObject, TokenValue, Token};
use super::ast_types::*;
use super::errorq::*;
use super::Program;

mod statement_parser;
mod experssion_parser;
mod type_parser;

use statement_parser::*;
use experssion_parser::*;
use type_parser::*;

pub type StmtHandler = fn(String, &mut Parser) -> Result<(String, StatementEnum), String>;

pub type NudHandler = fn(&mut Parser) -> ExpressionEnum;
pub type LedHandler = fn(&mut Parser, ExpressionEnum, BindingPower) -> ExpressionEnum;
pub type TypeNudHandler = fn(&mut Parser) -> TypesEnum;

use super::lexer::objects::Position;

pub struct Parser {
    pub position: Position,
    pub errors: ParserError,
    curr_token: TokenObject, // keep this for statements like LET and CONST where it'll be nice to know what was there.
}

// Just cancel this whole file!

impl Parser {
    fn new() -> Self {
        Self {
            position: Position::new(),
            errors: ParserError::new(),
            curr_token: TokenObject::new(Token::START, Position::new())
        }
    }

    fn get_next_token(&mut self, mut src_code: String) -> String {
        (src_code, self.curr_token) = super::get_next_token(src_code, self);

        self.curr_token.print_self();
        return src_code;
    }

    fn get_stmt_fn(&mut self, mut src_code: String) -> (String, Option<StmtHandler>) {
        src_code = self.get_next_token(src_code);

        return (src_code, self.curr_token.get_token().stmt());
    }

    fn expect(&mut self, expected_token: Token, error: ParserErrorTypes, mut src_code: String) -> Result<String, String> {
        src_code = self.get_next_token(src_code);

        if self.curr_token.get_token() != expected_token { 
            self.errors.add_error(error, self.position);

            return Err(src_code);
        }

        return Ok(src_code);

        // return (src_code, self.curr_token.get_token() == expected_token);
    }
}

fn get_name_from_symbol(token_value: TokenValue) -> String {
    match token_value {
        TokenValue::String(val) => val,
        _ => panic!("What sort of sorcery is this?!")
    }
}

pub fn parse(mut src_code: String) {
    let mut parser = Parser::new();
    let mut program = Program::new();

    let mut log_error = true;

    while !src_code.is_empty() {
        let (haystack, stmt_result) = parse_statement(src_code, &mut parser, log_error);
        src_code = haystack;

        log_error = false;

        if stmt_result.is_some() {
            log_error = true;
            program.add_statement(stmt_result.unwrap());
            // hmmm, i can get the global scope from here y'know...
        }
    }

    parser.errors.print_errors();

    println!("\n{:#?}", program);
}

fn parse_statement(mut src_code: String, parser: &mut Parser, log_error: bool) -> (String, Option<StatementEnum>) {
    let (haystack, stmt_fn) = parser.get_stmt_fn(src_code); // the position will be passed internally.
    src_code = haystack;

    if stmt_fn.is_none() {
        if log_error { parser.errors.add_error(ParserErrorTypes::Expects_Stmt_Fn, parser.position); }
        return (src_code, None);
    }

    let result = (stmt_fn.unwrap())(src_code, parser);
    if result.is_err() {
        let src_code = result.unwrap_err();
        return (src_code, None);
    } else {
        let (src_code, stmt) = result.unwrap();
        return (src_code, Some(stmt));
    }
}

fn parse_struct_declaration_statement(mut src_code: String, parser: &mut Parser) -> Result<(String, StatementEnum), String> {
    let mut result: bool;

    src_code = parser.expect(Token::SYMBOL, ParserErrorTypes::Expects_Name, src_code)?;
    let struct_name = get_name_from_symbol(parser.curr_token.take_value());

    // (src_code, result) = parser.expect(Token::OPEN_CURLY, ParserErrorTypes::Expects_Token("{"), src_code);
    // if !result { return (src_code, None); }

    // let mut struct_fields: HashMap<String, TypesEnum> = HashMap::new();
    // src_code = parser.get_next_token(src_code);

    // while parser.curr_token.get_token() != Token::CLOSE_CURLY {

    // }

    return Ok((src_code, return_struct_declaration_statement(struct_name, HashMap::new())));

    // return (src_code, Some(return_struct_declaration_statement(struct_name, HashMap::new())));
}


// pub fn parse_struct_declaration_stdatement(parser: &mut Parser) -> Vec<StatementEnum> {
   
    

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


// pub fn parse(lexer_result: (VecDeque<TokenObject>, LexerError)) {
//     let mut parser = Parser::new(lexer_result);
    

//     // while parser.has_tokens() {
//     //     // let global_statement = parse_statement(&mut parser);
//     //     // parser.program.add_statement(global_statement);
//     // }


//     println!("\n{:#?}", parser.program); // that's if there aren't any tokens
// }

pub struct Parser2 {
    tokens: VecDeque<TokenObject>,
    lexer_errors: LexerError,

    pub program: Program,
    pub parser_errors: ParserError
}

impl Parser2 {
    pub fn new(lexer_result: (VecDeque<TokenObject>, LexerError)) -> Self {
        let (tokens, lexer_errors) = lexer_result;

        Self {
            tokens,
            lexer_errors,
            program: Program::new(),
            parser_errors: ParserError::new(),
        }
    }

    pub fn get_current_token(&self) -> Token {
        self.tokens[0].get_token()
    }

    pub fn has_tokens(&self) -> bool {
        self.get_current_token() != Token::EOF
    }

    // pub fn expect(&mut self, expected_token: Token) -> Option<TokenObject> {
    //     // if it's not the token we expect, don't pop it from the list
        
    //     if self.get_current_token() != expected_token {
    //         self.parser_errors.add_error(ParserErrorTypes::Random);
    //         return None;
    //     } else {
    //         return Some(self.tokens.pop_front().unwrap());
    //     }
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum BindingPower {
    None,
    DefaultBp,
    Assignment,
    Logical,
    Relational,
    Additive,
    Multiplicative,
    Unary,
    Call,
    Member,
    Primary,
}

impl Token {
    pub const fn stmt(&self) -> Option<StmtHandler> {
        match self {
            Token::STRUCT => Some(parse_struct_declaration_statement),
            _ => None
        }
    }
}



// impl Token {
//     pub const fn binding_power(&self) -> BindingPower {
//         match *self {
//             Token::ASSIGN | Token::PLUS_ASSIGN | Token::MINUS_ASSIGN => BindingPower::Assignment,

//             Token::STAR | Token::DIVIDE | Token::PERCENT => BindingPower::Multiplicative,
//             Token::PLUS | Token::MINUS => BindingPower::Additive,

//             // Token::LESS | Token::LESS_EQUALS => BindingPower::Relational,
//             // Token::GREATER | Token::GREATER_EQUALS => BindingPower::Relational,
//             // Token::EQUALS | Token::NOT_EQUALS => BindingPower::Relational,

//             // Token::AND | Token::OR => BindingPower::Logical,

//             _=> BindingPower::None
//         }
//     }

//     pub const fn nud(&self) -> Option<NudHandler> {
//         match *self {
//             Token::NUMBER | Token::STRING | Token::SYMBOL => Some(parse_primary_expression),
//             Token::OPEN_BRACKET => Some(parse_bracket_expression),
//             Token::OPEN_SQUARE => Some(parse_array_expression),
//             Token::MINUS | Token::PLUS => Some(parse_prefix_expression),
//             _ => None,
//         }
//     }

//     pub const fn led(&self) -> Option<LedHandler> {
//         match self.binding_power() {
//             /* BindingPower::Logical | BindingPower::Relational | */ BindingPower::Additive | BindingPower::Multiplicative => Some(parse_binary_expression),
//             BindingPower::Assignment => Some(parse_assignment_expression),
//             _ => None,
//         }
//     }

//     pub const fn stmt(&self) -> Option<StmtHandler> {
//         match self {
//             Token::LET | Token::CONST => Some(parse_variable_declaration_statement),
//             Token::STRUCT => Some(parse_struct_declaration_statement),
//             Token::MONK => Some(parse_function_declaration_statement),
//             _ => None,
//         }
//     }

//     pub const fn nud_type(&self) -> Option<TypeNudHandler> {
//         match *self {
//             Token::SYMBOL => Some(parse_primary_type),
//             Token::OPEN_SQUARE => Some(parse_array_type),
//             Token::OPEN_BRACKET => Some(parse_tuple_type),
//             _ => None,
//         }
//     }
// }