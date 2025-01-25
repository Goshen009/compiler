






use std::collections::VecDeque;
use super::tokens::{TokenObject, Token};
use super::ast_types::*;
use super::errorq::*;
use super::Program;

mod statement_parser;
mod experssion_parser;
mod type_parser;

use statement_parser::*;
use experssion_parser::*;
use type_parser::*;

pub type NudHandler = fn(&mut Parser) -> ExpressionEnum;
pub type LedHandler = fn(&mut Parser, ExpressionEnum, BindingPower) -> ExpressionEnum;
pub type TypeNudHandler = fn(&mut Parser) -> TypesEnum;

pub fn parse(lexer_result: (VecDeque<TokenObject>, LexerError)) {
    let mut parser = Parser::new(lexer_result);

    // while parser.has_tokens() {
    //     // let global_statement = parse_statement(&mut parser);
    //     // parser.program.add_statement(global_statement);
    // }


    println!("\n{:#?}", parser.program); // that's if there aren't any tokens
}

pub struct Parser {
    tokens: VecDeque<TokenObject>,
    lexer_errors: LexerError,

    pub program: Program,
    pub parser_errors: ParserError
}

impl Parser {
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
    pub const fn stmt(&self) -> Result<statement_parser::StmtHandler, ()> {
        match self {
            // Token::LET | Token::CONST => Ok(parse_variable_declaration_statement),
            // Token::STRUCT => Ok(parse_struct_declaration_statement),
            // Token::MONK => Ok(parse_function_declaration_statement),
            _ => Err(()),
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