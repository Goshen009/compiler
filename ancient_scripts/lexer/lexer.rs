use std::collections::VecDeque;
use regex::Regex;
use tokens::{Token, TokenStruct, TokenValue};
use crate::errorq::{LexerError, LexerErrorTypes};

pub mod tokens;

struct LexerObj {
    curr_line: usize,
    curr_column: usize,
    patterns: Vec<(String, fn(Token, &str, &LexerObj) -> Option<TokenStruct>, Token)>,
    keywords: Vec<(String, Token)>,
}

impl LexerObj {
    fn new() -> Self {
        Self { curr_line: 0, curr_column: 0, patterns: get_patterns(), keywords: get_keywords() }
    }
}

fn symbol_or_keyword_handler(_input_token: Token, matched_string: &str, lex_obj: &LexerObj) -> Option<TokenStruct> {
    for (pattern, token) in &lex_obj.keywords {
        let re = Regex::new(pattern).unwrap();
        let is_matched = re.is_match(matched_string);
        
        if is_matched {
            return Some(TokenStruct {
                token: *token,
                value: TokenValue::None,
                line: lex_obj.curr_line,
                column: lex_obj.curr_column,
            });
        }
    }

    Some(TokenStruct {
        token: Token::SYMBOL,
        value: TokenValue::String(matched_string.to_string()),
        line: lex_obj.curr_line,
        column: lex_obj.curr_column,
    })
}

fn number_handler(_input_token: Token, matched_string: &str, lex_obj: &LexerObj) -> Option<TokenStruct> {
    let num_value = matched_string.parse().unwrap();
    Some(TokenStruct {
        token: Token::NUMBER,
        value: TokenValue::Number(num_value),
        line: lex_obj.curr_line,
        column: lex_obj.curr_column,
    })
}

fn string_handler(_input_token: Token, matched_string: &str, lex_obj: &LexerObj) -> Option<TokenStruct> {
    let len = matched_string.len() - 1;
    let value = &matched_string[1..len];

    Some(TokenStruct {
        token: Token::STRING,
        value: TokenValue::String(value.to_string()),
        line: lex_obj.curr_line,
        column: lex_obj.curr_column,
    })
}

fn default_handler(input_token: Token, _matched_string: &str, lex_obj: &LexerObj) -> Option<TokenStruct> {
    Some(TokenStruct {
        token: input_token,
        value: TokenValue::None,
        line: lex_obj.curr_line,
        column: lex_obj.curr_column,
    })
}

fn patterns(haystack: &str, lex_obj: &LexerObj) -> (Option<TokenStruct>, usize) {
    let mut return_token = None;
    let mut end: usize = 0;

    for (pattern, handler, token) in &lex_obj.patterns {
        let re = Regex::new(&pattern).unwrap();
        let is_matched = re.is_match(haystack);
        
        if is_matched {
            let captured_group = re.captures(haystack);
            end = captured_group.as_ref().unwrap().get(0).unwrap().end();

            let value = captured_group.unwrap().get(0).unwrap();
            let matched_string = value.as_str().trim();

            return_token = handler(*token, matched_string, lex_obj);
            break;
        }
    }

    (return_token, end)
}

pub fn lex(src_code: String) -> (VecDeque<TokenStruct>, LexerError) {
    let mut tokens:VecDeque<TokenStruct> = VecDeque::new();

    let mut lex_obj = LexerObj::new();
    let mut lex_errors = LexerError::new();
    
    'main_loop: for line in src_code.lines() {
        lex_obj.curr_line += 1;

        let mut start = 0;
        let src_len = line.len();

        while start < src_len {
            let haystack = &line[start..];
            lex_obj.curr_column = start;
            
            let (token_option, end) = patterns(haystack, &lex_obj);
            if let None = token_option {
                let error = LexerErrorTypes::Match_Not_Found { line: lex_obj.curr_line, column: lex_obj.curr_column };
                lex_errors.add_error(error);

                continue 'main_loop;
            }
            
            if let Some(token) = token_option {
                if token.get_token() == Token::COMMENT {
                    continue 'main_loop;
                } else if token.get_token() == Token::SPACE{
                    start += end;
                } else {
                    token.print_token_value();
                    start += end;
                    tokens.push_back(token);
                }
            }
        }
    }

    tokens.push_back(TokenStruct {
        token: Token::EOF,
        value: TokenValue::None,
        line: lex_obj.curr_line,
        column: 0,
    });

    (tokens, lex_errors)
}

fn get_keywords() -> Vec<(String, Token)> {
    let keyword: Vec<(String, Token)> = vec! [
        (String::from(r"^(let)\b"), Token::LET),
        (String::from(r"^(const)\b"), Token::CONST),
        (String::from(r"^(return)\b"), Token::RETURN),
        (String::from(r"^(scream)\b"), Token::SCREAM),
        (String::from(r"^(struct)\b"), Token::STRUCT),
        (String::from(r"^(monk)\b"), Token::MONK),
    ];

    keyword
}

fn get_patterns() -> Vec<(String, fn(Token, &str, &LexerObj) -> Option<TokenStruct>, Token)> {
    let patterns: Vec<(String, fn(Token, &str, &LexerObj) -> Option<TokenStruct>, Token)> = vec![
        (String::from(r"^(\s+)"), default_handler, Token::SPACE),
        (String::from(r"^\s*(\/\/)"), default_handler, Token::COMMENT),

        (String::from("^\\s*\"()[^\"]*\""), string_handler, Token::STRING),
        (String::from(r"^\s*([a-zA-Z_][a-zA-Z0-9_]*)"), symbol_or_keyword_handler, Token::SYMBOL),
        (String::from(r"^\s*([0-9]+)\b"), number_handler, Token::NUMBER),

        (String::from(r"^\s*(\&&)"), default_handler, Token::AND),
        (String::from(r"^\s*(\|\|)"), default_handler, Token::OR),

        (String::from(r"^\s*(\+=)"), default_handler, Token::PLUS_ASSIGN),
        (String::from(r"^\s*(\-=)"), default_handler, Token::MINUS_ASSIGN),
        (String::from(r"^\s*(\->)"), default_handler, Token::DASH_GREATER),

        (String::from(r"^\s*(>=)"), default_handler, Token::GREATER_EQUALS),
        (String::from(r"^\s*(<=)"), default_handler, Token::LESS_EQUALS),
        (String::from(r"^\s*(!=)"), default_handler, Token::NOT_EQUALS),
        (String::from(r"^\s*(==)"), default_handler, Token::EQUALS),
        (String::from(r"^\s*(>)"), default_handler, Token::GREATER),
        (String::from(r"^\s*(<)"), default_handler, Token::LESS),

        (String::from(r"^\s*(\()"), default_handler, Token::OPEN_BRACKET),
        (String::from(r"^\s*(\))"), default_handler, Token::CLOSE_BRACKET),
        (String::from(r"^\s*(\{)"), default_handler, Token::OPEN_CURLY),
        (String::from(r"^\s*(\})"), default_handler, Token::CLOSE_CURLY),
        (String::from(r"^\s*(\[)"), default_handler, Token::OPEN_SQUARE),
        (String::from(r"^\s*(\])"), default_handler, Token::CLOSE_SQUARE),

        (String::from(r"^\s*(\,)"), default_handler, Token::COMMA),
        (String::from(r"^\s*(\:)"), default_handler, Token::COLON),
        (String::from(r"^\s*(\;)"), default_handler, Token::SEMICOLON),
        (String::from(r"^\s*(\!)"), default_handler, Token::NOT),
        (String::from(r"^\s*(\=)"), default_handler, Token::ASSIGN),
        (String::from(r"^\s*(\-)"), default_handler, Token::MINUS),
        (String::from(r"^\s*(\/)"), default_handler, Token::DIVIDE),
        (String::from(r"^\s*(\+)"), default_handler, Token::PLUS),
        (String::from(r"^\s*(\*)"), default_handler, Token::STAR),
        (String::from(r"^\s*(\%)"), default_handler, Token::PERCENT),
        (String::from(r"^\s*(\.)"), default_handler, Token::PERIOD),
    ];

    patterns
}