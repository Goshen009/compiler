use std::{iter::Peekable, collections::VecDeque};
use crate::errorq::LexerError;
use super::{Token, TokenObject};

#[derive(Debug, Copy, Clone)]
pub struct Position { 
    pub line: usize, 
    pub column: usize 
}

impl Position {
    pub fn new() -> Self { Self { line: 0, column: 0 } }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "at (Ln {}, Col {})", self.line, self.column)
    }
}

pub struct Lexer { // I won't be passing this on. Only the VecDeque<> and Errors
    src: Peekable<std::vec::IntoIter<String>>,
    pub errors: LexerError,
    pub curr_position: Position,
    pub tokens: VecDeque<TokenObject>,
}

impl Lexer {
    pub fn new(src: Peekable<std::vec::IntoIter<String>>) -> Self { 
        let new_lexer = Self {
            src,
            tokens: VecDeque::from(vec![TokenObject::new(Token::START, Position::new())]),
            errors: LexerError::new(),
            curr_position: Position::new(),
        };

        new_lexer.tokens[0].print_self();
        return new_lexer;
    }

    pub fn get_token_at(&self, index: &usize) -> &TokenObject {
        return self.tokens.get(*index).unwrap();
    }

    pub fn get_current_index(&self) -> usize {
        return self.tokens.len() - 1;
    }

    pub fn has_next_line(&mut self) -> bool {
        return self.src.peek().is_some();
    }

    pub fn initialize_new_line(&mut self) -> String {
        if self.has_next_line() {
            self.curr_position.new_line();
            return self.src.next().unwrap();
        } else {
            return String::new();
        }
    }

    // pub fn return_matched_word(&mut self, index: usize) {
    //     let remaining_string_on_line = self.src_line.split_off(index);
    //     // let matched_word = self.src_line;

    //     self.src_line = remaining_string_on_line;

    //     let t: String = self.src_line.drain(..10).collect();

    //     self.src_line.
    //     // return self.src_line;
    // }

    pub fn add_token(&mut self, mut token: TokenObject) {
        if token.get_token() == Token::SPACE {
            return; // SPACE tokens should not be in the tokens list.
        }

        if !self.tokens.is_empty() { 
            super::handlers::check_token_syntax(&mut token, self);
        }

        token.print_self();
        self.tokens.push_back(token);
    }

    pub fn completed_without_errors(&mut self) -> bool {
        super::handlers::grouping_handlers::check_grouping_at_EOF(self);

        if self.errors.has_errors() {
            self.errors.print_errors(&self);
            return false;
        }

        return true;
    }
}