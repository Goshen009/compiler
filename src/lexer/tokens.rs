use super::objects::Position;
use logos::{Lexer, Logos, Skip};
use regex::Regex;

#[derive(Default)]
pub struct LexerState {
    pub position: Position,
    pub value: String,
    last_new_line_position: usize,
    next_line: usize
}

#[allow(non_camel_case_types)]
#[derive(Debug, Logos, Copy, Clone, PartialEq, Eq, Hash)]
#[logos(extras = LexerState)]
pub enum Token {
    #[regex(r#""[^"]*""#, handle_string)]
    STRING,
    // it remains the handler and the errors for invalid strings
    // if it does not close, then it'll be an error token that consumes the 
    // rest of the src_code
    
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", move_position)]
    SYMBOL, needs to have a handler that will get the symbol

    #[regex(r"[0-9]+", move_position)]
    NUMBER, will require a callback that will add it's value

    // make the keywords have word boundaries.

    #[token("//", handle_comment)]
    COMMENT,

    #[token("+=", move_position)]
    PLUS_ASSIGN,

    #[token("-=", move_position)]
    MINUS_ASSIGN,

    #[token("->", move_position)]
    DASH_GREATER,

    #[token("&&", move_position)]
    AND,
    
    #[token("||", move_position)]
    OR,
    
    #[token(">=", move_position)]
    GREATER_EQUALS,
    
    #[token("<=", move_position)]
    LESS_EQUALS,
    
    #[token("!=", move_position)]
    NOT_EQUALS,
    
    #[token("==", move_position)]
    EQUALS,
    
    #[token(">", move_position)]    
    GREATER,
    
    #[token("<", move_position)]
    LESS,
    
    #[token("(", move_position)]
    OPEN_BRACKET,

    #[token(")", move_position)]
    CLOSE_BRACKET,

    #[token("{", move_position)]
    OPEN_CURLY,

    #[token("}", move_position)]
    CLOSE_CURLY,

    #[token("[", move_position)]
    OPEN_SQUARE,

    #[token("]", move_position)]
    CLOSE_SQUARE,

    #[token(",", move_position)]
    COMMA,

    #[token(":", move_position)]
    COLON,

    #[token(";", move_position)]
    SEMICOLON,

    #[token("!", move_position)]
    NOT,

    #[token("-", move_position)]
    MINUS,

    #[token("/", move_position)]
    DIVIDE,

    #[token("=", move_position)]
    ASSIGN,

    #[token("+", move_position)]
    PLUS,

    #[token("*", move_position)]
    STAR,

    #[token("%", move_position)]
    PERCENT,

    #[token(".", move_position)]
    PERIOD,
    
    #[token("let", move_position)]
    LET,

    #[token("const", move_position)]
    CONST,

    #[token("return", move_position)]
    RETURN,

    #[token("scream", move_position)]
    SCREAM,

    #[token("struct", move_position)]
    STRUCT,

    #[token("monk", move_position)]
    MONK,

    #[token("if", move_position)]
    IF,

    #[token("else", move_position)]
    ELSE,

    #[regex(r"[^\S\r\n]+", skip_space)]
    SPACE,

    #[regex(r"\r?\n", move_to_new_line)]
    NEW_LINE,
}

fn move_position(lexer: &mut Lexer<Token>) {
    lexer.extras.position.line += lexer.extras.next_line;
    lexer.extras.next_line = 0;
    
    lexer.extras.position.column = (lexer.span().start - lexer.extras.last_new_line_position) + 1;
    lexer.extras.value = String::new();
}

fn skip_space(lexer: &mut Lexer<Token>) -> Skip {
    move_position(lexer);
    Skip
}

fn move_to_new_line(lexer: &mut Lexer<Token>) -> Skip {
    lexer.extras.position.line += lexer.extras.next_line;
    lexer.extras.next_line = 0;

    lexer.extras.position.line += 1;
    lexer.extras.position.column = 0;
    lexer.extras.last_new_line_position = lexer.span().end;

    Skip
}

fn handle_comment(lexer: &mut Lexer<Token>) -> Skip {
    let regex = Regex::new(r"(\r?\n)").unwrap();

    if let Some(mat) = regex.find(lexer.remainder()) {
        let new_position = mat.start();
        lexer.bump(new_position);
    } else {
        let new_position = lexer.remainder().len();
        lexer.bump(new_position);
    }

    Skip
}

fn handle_string(lexer: &mut Lexer<Token>) {
    let regex = Regex::new(r"(\r?\n)").unwrap();

    if let Some((index, last_match)) = regex.find_iter(lexer.slice()).enumerate().last() {
        lexer.extras.next_line = index + 1;
        lexer.extras.last_new_line_position = lexer.span().end + last_match.end();
    }
    
    let slice = lexer.slice();
    lexer.extras.value = slice[1..slice.len()-1].to_string();

    lexer.extras.position.column = (lexer.span().start - lexer.extras.last_new_line_position) + 1;
}