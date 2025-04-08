use std::collections::VecDeque;
use super::tokens::Token;

#[derive(Debug)]
pub struct Position { 
    pub line: usize, 
    pub column: usize 
}

impl Default for Position {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

pub struct TokenObject {
    token: Token,
    value: String,
    position: Position
}

#[derive(Default)]
pub struct LexerObject {
    errors: Vec<String>,
    pub tokens: VecDeque<TokenObject>,
}