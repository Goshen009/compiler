use crate::parser::{LedHandler, NudHandler, StmtHandler, TypeNudHandler};
use crate::parser::*;

#[derive(Debug)]
pub enum TokenValue {
    String(String),
    Number(i32),
    None,
}

pub struct TokenStruct {
    pub token: Token,
    pub value: TokenValue,
    pub line: usize,
    pub column: usize
}

impl TokenStruct {
    pub fn get_token(&self) -> Token {
        self.token
    }

    pub fn check_token_value(&self) -> &TokenValue {
        &self.value
    }

    pub fn print_token_value(&self) {
        match &self.value {
            TokenValue::None => println!("{:?}", self.token),
            TokenValue::Number(val) => println!("{:?} ({val})", self.token),
            TokenValue::String(val) => println!(r#"{:?} ("{val}")"#, self.token),           
        }
    }

    pub fn get_token_value(self) -> TokenValue {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum BindingPower {
    None,
    DefaultBp,
    //Grouping,
    Assignment,
    // StructInit,
    Logical,
    Relational,
    Additive,
    Multiplicative,
    Unary,
    Call,
    Member,
    Primary,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    COMMENT,        // well..

    STRING,         // string
    SYMBOL,         // var_names
    NUMBER,         // integer number

    LET,            // let
    CONST,          // const
    RETURN,         // return
    SCREAM,         // scream
    STRUCT,         // struct
    MONK,           // monk
    IF,             // if
    ELSE,           // else

    PLUS_ASSIGN,    // +=
    MINUS_ASSIGN,   // -=
    DASH_GREATER,   // ->

    AND,            // &&
    OR,             // ||
    
    GREATER_EQUALS, // >=
    LESS_EQUALS,    // <=
    NOT_EQUALS,     // !=
    EQUALS,         // ==
    GREATER,        // >
    LESS,           // <
    
    OPEN_BRACKET,   // (
    CLOSE_BRACKET,  // )
    OPEN_CURLY,     // {
    CLOSE_CURLY,    // }
    OPEN_SQUARE,    // [
    CLOSE_SQUARE,   // ]

    COMMA,          // ,
    COLON,          // :
    SEMICOLON,      // ;
    NOT,            // !
    MINUS,          // -
    DIVIDE,         // /
    ASSIGN,         // =
    PLUS,           // +
    STAR,           // *
    PERCENT,        // %
    PERIOD,         // .

    SPACE,          // not in use
    EOF,            // end of file
}

impl Token {
    pub const fn binding_power(&self) -> BindingPower {
        match *self {
            Token::ASSIGN | Token::PLUS_ASSIGN | Token::MINUS_ASSIGN => BindingPower::Assignment,

            Token::STAR | Token::DIVIDE | Token::PERCENT => BindingPower::Multiplicative,
            Token::PLUS | Token::MINUS => BindingPower::Additive,

            // Token::LESS | Token::LESS_EQUALS => BindingPower::Relational,
            // Token::GREATER | Token::GREATER_EQUALS => BindingPower::Relational,
            // Token::EQUALS | Token::NOT_EQUALS => BindingPower::Relational,

            // Token::AND | Token::OR => BindingPower::Logical,

            _=> BindingPower::None
        }
    }

    pub const fn nud(&self) -> Option<NudHandler> {
        match *self {
            Token::NUMBER | Token::STRING | Token::SYMBOL => Some(parse_primary_expression),
            Token::OPEN_BRACKET => Some(parse_bracket_expression),
            Token::OPEN_SQUARE => Some(parse_array_expression),
            Token::MINUS | Token::PLUS => Some(parse_prefix_expression),
            _ => None,
        }
    }

    pub const fn led(&self) -> Option<LedHandler> {
        match self.binding_power() {
            /* BindingPower::Logical | BindingPower::Relational | */ BindingPower::Additive | BindingPower::Multiplicative => Some(parse_binary_expression),
            BindingPower::Assignment => Some(parse_assignment_expression),
            _ => None,
        }
    }

    pub const fn stmt(&self) -> Option<StmtHandler> {
        match self {
            Token::LET | Token::CONST => Some(parse_variable_declaration_statement),
            Token::STRUCT => Some(parse_struct_declaration_statement),
            Token::MONK => Some(parse_function_declaration_statement),
            _ => None,
        }
    }

    pub const fn nud_type(&self) -> Option<TypeNudHandler> {
        match *self {
            Token::SYMBOL => Some(parse_primary_type),
            Token::OPEN_SQUARE => Some(parse_array_type),
            Token::OPEN_BRACKET => Some(parse_tuple_type),
            _ => None,
        }
    }
}