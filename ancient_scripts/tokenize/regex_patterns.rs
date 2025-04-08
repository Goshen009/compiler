





use std::collections::HashMap;
use std::sync::LazyLock;
use regex::Regex;

pub fn check_if_whitespace(char: &String) -> bool {
    Regex::new(r"\s").unwrap().is_match(char)
}

pub fn check_pattern_match(pattern: &str, current_char: &String) -> bool {
    Regex::new(pattern).unwrap().is_match(current_char)
}

pub static WORD_CHARACTER_REGEX_WITH_NUMBER: &str = r"[a-zA-Z0-9_]";
pub static WORD_CHARACTER_REGEX: &str = r"[a-zA-Z_]";
pub static NON_WORD_CHARACTER_REGEX: &str = r"\W";

pub static KEYWORD_REGEX: LazyLock<HashMap<Token, &str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(Token::Int, r"^int$");
    map.insert(Token::Return, r"^return$");
    map.insert(Token::If, r"^if$");
    map.insert(Token::Shout, r"^shout$");
    map.insert(Token::Fn, r"^fn$");

    map
});

pub static SINGLE_CHAR_REGEX: LazyLock<HashMap<Token, &str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(Token::OpenCurly, r"\{");
    map.insert(Token::CloseCurly, r"\}");
    map.insert(Token::OpenBrackets, r"\(");
    map.insert(Token::CloseBrackets, r"\)");
    map.insert(Token::Divide, r"\/");
    map.insert(Token::Add, r"\+");
    map.insert(Token::Subtract, r"\-");
    map.insert(Token::Multiply, r"\*");
    map.insert(Token::Period, r"\.");
    map.insert(Token::Assign, r"\=");
    map.insert(Token::Comma, r"\,");
    map.insert(Token::SemiColon, r"\;");
    map.insert(Token::Quotation, r#"""#);

    map
});

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Token {
    EOF,
    Variable(String),

    OpenCurly,
    CloseCurly,
    OpenBrackets,
    CloseBrackets,
    Divide,
    Add,
    Subtract,
    Multiply,
    Period,
    Assign,
    Comma,
    SemiColon,

    IsEquals,
    Comments,

    Int,
    Return,
    If,
    Shout,
    Quotation,
    Fn,
}

impl Token {
    pub fn new(&self) -> Self {
        match self {
            Token::EOF => Token::EOF,
            Token::Variable(_) => Token::Variable(String::new()),
            Token::OpenCurly => Token::OpenCurly,
            Token::CloseCurly => Token::CloseCurly,
            Token::OpenBrackets => Token::OpenBrackets,
            Token::CloseBrackets => Token::CloseBrackets,
            Token::Divide => Token::Divide,
            Token::Add => Token::Add,
            Token::Subtract => Token::Subtract,
            Token::Multiply => Token::Multiply,
            Token::Period => Token::Period,
            Token::Assign => Token::Assign,
            Token::Comma => Token::Comma,
            Token::SemiColon => Token::SemiColon,
            Token::IsEquals => Token::IsEquals,
            Token::Comments => Token::Comments,
            Token::Int => Token::Int,
            Token::Return => Token::Return,
            Token::If => Token::If,
            Token::Shout => Token::Shout,
            Token::Quotation => Token::Quotation,
            Token::Fn => Token::Fn,
        }
    }
}