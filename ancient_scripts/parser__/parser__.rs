use crate::{ast_types::*, ParserError, ParserErrorTypes, Position};
use std::collections::{HashMap, VecDeque};
use crate::lexer::tokens::{Token, TokenStruct, TokenValue, BindingPower};

pub use statements_parser::*;
pub use expressions_parser::*;
pub use types_parser::*;

pub mod statements_parser;
pub mod expressions_parser;
pub mod types_parser;

pub type StmtHandler = fn(&mut Parser) -> Vec<StatementEnum>;
pub type NudHandler = fn(&mut Parser) -> ExpressionEnum;
pub type LedHandler = fn(&mut Parser, ExpressionEnum, BindingPower) -> ExpressionEnum;
pub type TypeNudHandler = fn(&mut Parser) -> TypesEnum;

pub struct Parser {
    pub tokens: VecDeque<TokenStruct>,
    pub errors: ParserError
}

impl Parser {
    pub fn get_current_token(&self) -> Token {
        self.tokens[0].get_token()
    }

    pub fn get_current_token_position(&self) -> Position {
        Position { line: self.tokens[0].line, column: self.tokens[0].column }
    }

    pub fn get_next_token(&self) -> Token {
        if self.tokens.len() >= 2 {
            self.tokens[1].get_token()
        } else {
            panic!("No more tokens!")
        }        
    }

    pub fn has_token(&self) -> bool {
        self.get_current_token() != Token::EOF
    }

    pub fn advance(&mut self) -> TokenStruct {
        self.tokens.pop_front().unwrap()
    }

    pub fn expect_error(&mut self, expected_token: Token, err_msg: &str) -> TokenStruct {
        let token = self.get_current_token();
        if token != expected_token {
            panic!("{}", err_msg);
        }

        self.advance()
    }

    pub fn expect(&mut self, expected_token: Token) -> TokenStruct {
        let err_msg = format!("Expected a {:?} but found {:?}", expected_token, self.get_current_token());
        self.expect_error(expected_token, &err_msg)
    }
}

pub fn parse(tokens: VecDeque<TokenStruct>) -> Program {
    let mut program = Program {
        code: Vec::new()
    };

    let mut parser = Parser {
        tokens,
        errors: ParserError::new()
    };

    while parser.has_token() {
        // code.push(parse_statement(&mut parser));
        for statement in parse_statement(&mut parser) {
            program.code.push(statement);
        }
    }

    println!("\n{:#?}", program);
    return program;
}

fn get_name_from_symbol(token_value: TokenValue) -> String {
    match token_value {
        TokenValue::String(val) => val,
        _ => panic!("What sort of sorcery is this?!")
    }
}

pub fn expect_comma(parser: &mut Parser) {
    let is_final_iteration = parser.get_current_token() == Token::CLOSE_BRACKET;
    if !is_final_iteration {
        parser.expect_error(Token::COMMA, "FOOL! You forgot the comma at the end!");
    }
}