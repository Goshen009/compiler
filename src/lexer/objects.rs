use std::collections::VecDeque;
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

pub struct Lexer {
    errors: Vec<String>,
    pub curr_position: Position,
    pub tokens: VecDeque<TokenObject>,
}

impl Lexer {
    pub fn new(src_code: String) -> Self { 
        let mut lexer = Self {
            tokens: VecDeque::from(vec![TokenObject::new(Token::START, Position::new())]),
            errors: Vec::new(),
            curr_position: Position::new()
        };

        lexer.tokens[0].print_self();
        
        super::lex_tokens(&mut lexer, src_code);
        return lexer;
    }

    pub fn add_error(&mut self, error: String) {    
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn add_token(&mut self, mut token: TokenObject) {
        if token.get_token() == Token::SPACE {
            return;
        }

        token.print_self();
        self.tokens.push_back(token);
    }

    pub fn print_errors(&self) {
        println!("\nERRORS: ");
        for error in self.errors.iter() {
            println!("{error}");
        }
    }

    pub fn completed_without_errors(&self) -> bool {
        if self.has_errors() {
            self.print_errors();
        }
        return !self.has_errors();
    }
}