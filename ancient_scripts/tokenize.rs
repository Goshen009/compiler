mod regex_patterns;
use regex_patterns::*;

use std::iter::Peekable;
use std::str::Chars;

pub fn start_tokenization(src_code: &String) {
    let mut line_tokens: Vec<Token> = vec![];

    for line in src_code.lines() {
        line_tokens.extend(start_line_tokenization(&mut line.chars().peekable()));
    }
    line_tokens.push(Token::EOF);

    for token in line_tokens.iter() {
        println!("{:?}", token);
    }
}

fn start_line_tokenization(characters: &mut Peekable<Chars<'_>>) -> Vec<Token> {
    let mut line_tokens: Vec<Token> = vec![];

    let mut closure = || {
        if let Some(current_char) = characters.peek() {
            let current_char = &current_char.to_string();

            if check_if_whitespace(current_char) {
                characters.next();
                return true;
            }

            if check_pattern_match(NON_WORD_CHARACTER_REGEX, current_char) {
                if let Some(matched_token) = handle_char_regex(characters) {
                    line_tokens.push(matched_token);
                    return true;
                }
            }

            if check_pattern_match(WORD_CHARACTER_REGEX, current_char) {
                if let Some(matched_token) = handle_word_character_regex(characters) {
                    line_tokens.push(matched_token);
                    return true;
                }
            }

            println!("Unrecognized input!");
            characters.next();
            return true;
        } else {
            return false;
        }
    };

    while closure() {}

    line_tokens
}

fn handle_char_regex(characters: &mut Peekable<Chars<'_>>) -> Option<Token> {
    let mut return_token = None;

    if let Some(current_char) = characters.peek() {
        let current_char = &current_char.to_string();

        for (matched_token, pattern) in SINGLE_CHAR_REGEX.iter() {
            if check_pattern_match(pattern, current_char) {
                characters.next();
                return_token = Some(matched_token.new());
                break;
            }
        }
    }

    if let Some(matched_token) = &return_token {
        let rt = matched_token.new();

        if let Some(next_char) = characters.peek() {
            let next_char = &next_char.to_string();
            let pattern_check = check_pattern_match(SINGLE_CHAR_REGEX.get(&rt).unwrap(), next_char);

            if rt == Token::Assign && pattern_check {
                characters.next();
                return_token = Some(Token::IsEquals);
            }

            if rt == Token::Divide && pattern_check {
                characters.next();
                return_token = Some(Token::Comments);
            }
        }
    }

    return_token
}

fn handle_word_character_regex(characters: &mut Peekable<Chars<'_>>) -> Option<Token> {
    let mut return_token = None;
    let mut word = characters.next().unwrap().to_string();

    let mut closure = || {
        if let Some(next_char) = characters.peek() {
            let next_char = &next_char.to_string();

            if check_pattern_match(WORD_CHARACTER_REGEX_WITH_NUMBER, next_char) {
                word.push(characters.next().unwrap());
                return true;
            }
        }
        return false;
    };

    while closure() {}

    for (matched_token, pattern) in KEYWORD_REGEX.iter() {
        if check_pattern_match(pattern, &word) {
            return_token = Some(matched_token.new());
            break;
        }
    }

    if return_token == None {
        return_token = Some(Token::Variable(word));
    }

    return_token
}
