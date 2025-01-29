use std::collections::VecDeque;
use crate::errorq::LexerError;
use super::{Token, TokenObject};

#[derive(Debug, Copy, Clone)]
pub struct Position { 
    pub line: usize, 
    pub column: usize 
}

impl Position {
    pub fn new() -> Self { Self { line: 1, column: 1 } }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    pub fn move_line_by(&mut self, line_count: usize, column_count: usize) {
        self.line += line_count;

        if line_count == 0 { // if the string starts and ends on the same line, just add to it
            self.column += column_count;
        } else { // if it's multi-line, take the column and add 1 to it (because the column starts from position 1 not 0)
            self.column = 1 + column_count;
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Ln {}, Col {})", self.line, self.column)
    }
}

pub struct Lexer { // I won't be passing this on. Only the VecDeque<> and Errors
    pub errors: LexerError,
    pub curr_position: Position,
    pub tokens: VecDeque<TokenObject>,    
}

impl Lexer {
    pub fn new() -> Self { 
        let new_lexer = Self {
            tokens: VecDeque::from(vec![TokenObject::new(Token::START, Position::new())]),
            errors: LexerError::new(),
            curr_position: Position::new()
        };

        // new_lexer.tokens[0].print_self();
        return new_lexer;
    }

    pub fn get_token_at(&self, index: &usize) -> &TokenObject {
        return self.tokens.get(*index).unwrap();
    }

    pub fn get_current_index(&self) -> usize {
        return self.tokens.len() - 1;
    }

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
        // super::handlers::check_grouping_at_EOF(self);

        if self.errors.has_errors() {
            self.errors.print_errors();
            return false;
        }

        return true;
    }
}