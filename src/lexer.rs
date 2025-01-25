use regex::Regex;
use super::errorq::LexerErrorTypes;

pub mod tokens;
pub mod objects;
mod patterns;
mod handlers;

use tokens::*;
use objects::*;

pub fn lex(src_code: std::iter::Peekable<std::vec::IntoIter<String>>) {
    let mut lexer = Lexer::new(src_code);

    lex_tokens(&mut lexer, String::new());

    if lexer.completed_without_errors() { // moves on to the parser
        super::parse((lexer.tokens, lexer.errors));
    }
}

fn lex_tokens(lexer: &mut Lexer, mut haystack: String) {
    if haystack.is_empty() {
        if lexer.has_next_line() {
            haystack = lexer.initialize_new_line();
        } else {
            lexer.add_token(TokenObject::new(Token::EOF, lexer.curr_position));
            return;
        }
    }

    let mut found_match = false;

    for pattern in patterns::PATTERNS {
        let regex = Regex::new(pattern.pattern).unwrap();

        if regex.is_match(&haystack) {
            found_match = true;

            if pattern.token == Token::COMMENT { // if it's a comment, skip everything on that line.
                haystack = lexer.initialize_new_line();
                break;
            }

            if pattern.token == Token::DOUBLE_QUOTE {
                (_, haystack) = split_off(1, haystack); // remove the " from the start of the haystack so we can search on the rest.
                
                haystack = handle_string(lexer, haystack, String::new(), lexer.curr_position);
                break;
            }

            let matched_word;
            let matched_end_index = regex.captures(&haystack).unwrap().get(0).unwrap().end();

            let token_object = TokenObject::new(pattern.token, lexer.curr_position);
            lexer.curr_position.column += matched_end_index;

            (matched_word, haystack) = split_off(matched_end_index, haystack);

            let token = (pattern.handler)(token_object, matched_word);
            lexer.add_token(token);
            break;
        }
    }

    if !found_match && !haystack.is_empty() { // we'll get the next space character. That'll tell us the position at which the error word stops.
        let next_whitespace = haystack.find(|c: char| c.is_whitespace());
        let foreign_word;

        if let Some(index) = next_whitespace {
            (foreign_word, haystack) = split_off(index, haystack);
        } else {
            // if there are no more whitespaces, take the rest of the line as a foreign word and have the haystack be empty (i.e why the index is haystack.len())
            (foreign_word, haystack) = split_off(haystack.len(), haystack);
        }

        lexer.add_token(TokenObject::new(Token::ERROR, lexer.curr_position));
        lexer.curr_position.column += foreign_word.len();

        lexer.errors.add_error(LexerErrorTypes::Match_Not_Found { value: foreign_word, index: lexer.get_current_index() });
    }

    lex_tokens(lexer, haystack);
}

fn split_off(index: usize, mut haystack: String) -> (String, String) {
    let remainder = haystack.split_off(index);
    let matched_word = haystack;

    (matched_word, remainder)
}

fn handle_string(lexer: &mut Lexer, mut haystack: String, mut word: String, start_position: Position) -> String {
    let closing_quotes_index = haystack.find(|c: char| c == '"');

    match closing_quotes_index {
        Some(index) => {
            let found_word;

            (found_word, haystack) = split_off(index, haystack);
            word = format!("{}{}", word, found_word);

            let mut token_object = TokenObject::new(Token::STRING, start_position);
            token_object.update_token_value(TokenValue::String(word));

            lexer.curr_position.column += index + 1; // plus 1 for the last " character. or else the position will be off by one.
            lexer.add_token(token_object);

            (_, haystack) = split_off(1, haystack); // remove the " from the end of the haystack.
            return haystack;
        }

        None => {
            word = format!("{}{}", word, haystack); // add the current line (haystack) to the ongoing word.

            if lexer.has_next_line() {
                haystack = lexer.initialize_new_line();
                word.push('\n'); // adds a new line to the end of the word.

                return handle_string(lexer, haystack, word, start_position);
            } else {
                lexer.errors.add_error(LexerErrorTypes::Started_String_That_Was_Never_Ended(start_position));
                return String::new(); // returning a new string will make the lex() to try and get a new line. When it fails to, it'll call EOF and then the errors will print.
            }
        }
    }
}

pub fn default_handler(token_object: TokenObject, _matched_string: String) -> TokenObject {
    return token_object;
}

pub fn number_handler(mut token_object: TokenObject, matched_string: String) -> TokenObject {
    let value = matched_string.parse().unwrap();
    
    token_object.update_token_value(TokenValue::Number(value));
    return token_object;
}

pub fn symbol_or_keyword_handler(mut token_object: TokenObject, matched_string: String) -> TokenObject {
    for keyword in patterns::KEYWORDS {
        let regex = Regex::new(keyword.pattern).unwrap();

        if regex.is_match(&matched_string) {
            return TokenObject::new(keyword.token, token_object.get_position());
        }
    }

    let value = matched_string;
    token_object.update_token_value(TokenValue::String(value));
    return token_object;
}