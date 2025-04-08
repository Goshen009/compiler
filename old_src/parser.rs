use std::collections::VecDeque;
use super::{semantics::objects::Scope, errorq::*, ast_types::*, lexer::{tokens::*, objects::*}};

mod handlers;
mod statement_parser;
mod experssion_parser;
mod type_parser;

use statement_parser::*;
use experssion_parser::*;
use type_parser::*;

pub fn parse<'scope>(parser: &mut Parser, program: &mut Program, mut log_error: bool) -> Scope<'scope> {
    parser.advance(); // to remove the Token::START

    let mut global_scope = Scope::new(None);

    while parser.has_tokens() {
        let result = parse_statement(parser, log_error);
        log_error = result.is_some(); // we don't want to keep logging errors if the next token is not a statement token, so when it hits a wrong token, it'll log an error for just that one and search for the next stmt token.

        if result.is_some() {
            let stmt = result.unwrap();

            stmt.add_to_global_scope(&mut global_scope, parser); // this will add the error of another variable has the same name to the parser errors.
            program.add_statement(stmt);
        }
    }

    return global_scope;
}

pub fn get_name_from_symbol(token_value: TokenValue) -> String {
    match token_value {
        TokenValue::String(val) => val,
        _ => panic!("What sort of sorcery is this?!")
    }
}

pub struct Parser {
    pub errors: ParserError,
    tokens: VecDeque<TokenObject>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            tokens: lexer.tokens,
            errors: ParserError::new(),
        }
    }

    fn has_tokens(&self) -> bool {
        self.get_current_token() != Token::EOF
    }

    fn get_current_token(&self) -> Token {
        if self.tokens.len() == 0 {
            return Token::EOF;
        } else {
           return self.tokens[0].get_token();
        }
    }

    fn advance(&mut self) -> TokenObject {
        self.tokens.pop_front().unwrap()
    }

    fn log_error(&mut self, error: &str) -> Option<()> {
        self.errors.add_error(format!("{}", error));
        return None;
    }

    fn expect(&mut self, expected_token: Token, error: &str) -> Option<TokenObject> {
        let curr_token = self.advance();        
        if curr_token.get_token() != expected_token {
            self.errors.add_error(format!("{} at {}", error, curr_token.get_position()));
            return None;
        }

        return Some(curr_token);
    }

    pub fn completed_without_errors(&self, program: &Program) -> bool {
        if self.errors.has_errors() {
            self.errors.print_errors();
            return false;
        }

        println!("\n{:#?}", program);
        return true;
    }
}