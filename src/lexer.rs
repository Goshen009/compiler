use regex::Regex;
use super::errorq::LexerErrorTypes;

pub mod tokens;
pub mod objects;
mod patterns;
mod handlers;

use tokens::*;
use objects::*;

pub fn lex(src_code: String) {
    let mut lexer = Lexer::new();

    lex_tokens(&mut lexer, src_code);

    if lexer.completed_without_errors() { // moves on to the parser
        // super::parse((lexer.tokens, lexer.errors));
    }
}

fn lex_tokens(lexer: &mut Lexer, mut src_code: String) {
    if src_code.is_empty() { // there's nothing left in the source code.
        lexer.add_token(TokenObject::new(Token::EOF, lexer.curr_position));
        return;
    }

    let mut found_match = false;

    for pattern in patterns::PATTERNS {
        let regex = Regex::new(pattern.pattern).unwrap();

        if regex.is_match(&src_code) {
            found_match = true;

            if pattern.token == Token::COMMENT { // if it's a comment, skip everything on that line.
                src_code = comment_handler(src_code); // we're not starting a new line here because the comment handler returns the character of the new line so it'll match as new line on the next iteration.
                break;
            }

            if pattern.token == Token::DOUBLE_QUOTE {
                (_, src_code) = split_off(1, src_code); // remove the " from the start of the src_code so we can search on the rest.
                
                src_code = handle_string(lexer, src_code, lexer.curr_position);
                break;
            }

            let matched_word;
            let matched_end_index = regex.captures(&src_code).unwrap().get(0).unwrap().end();

            let token_object = TokenObject::new(pattern.token, lexer.curr_position);
            lexer.curr_position.column += matched_end_index;

            (matched_word, src_code) = split_off(matched_end_index, src_code);

            if pattern.token == Token::NEW_LINE {
                lexer.curr_position.new_line();
                break;
            }

            let token = (pattern.handler)(token_object, matched_word);
            lexer.add_token(token);
            break;
        }
    }

    if !found_match && !src_code.is_empty() { // we'll get the next space character. That'll tell us the position at which the error word stops.
        let next_whitespace = src_code.find(|c: char| c.is_whitespace());
        let foreign_word;

        if let Some(index) = next_whitespace {
            (foreign_word, src_code) = split_off(index, src_code);
        } else {
            // if there are no more whitespaces, take the rest of the line as a foreign word and have the src_code be empty (i.e why the index is src_code.len())
            (foreign_word, src_code) = split_off(src_code.len(), src_code);
        }

        lexer.add_token(TokenObject::new(Token::ERROR, lexer.curr_position));
        lexer.curr_position.column += foreign_word.len();
        
        let error = format!("Invalid token '{foreign_word}' at {}", lexer.curr_position);
        lexer.errors.add_error(error);
    }

    lex_tokens(lexer, src_code);
}

fn split_off(index: usize, mut src_code: String) -> (String, String) { //
    let remainder = src_code.split_off(index);
    let matched_word = src_code;

    (matched_word, remainder)
}

fn comment_handler(mut src_code: String) -> String {
    let mut lines = src_code.lines();
    let current_line = lines.next().unwrap();

    let end_of_current_line_index = current_line.len();
    (_, src_code) = split_off(end_of_current_line_index, src_code);

    return src_code;
}

fn handle_string(lexer: &mut Lexer, mut src_code: String, start_position: Position) -> String {
    let closing_quotes_index = src_code.find(|c: char| c == '"');

    match closing_quotes_index {
        None => {
            let error = format!("Started a string at {start_position} that was never ended");
            lexer.errors.add_error(error);
            return String::new(); // returning a new string will make the lex() to try and get a new line. When it fails to, it'll call EOF and then the errors will print.
        }

        Some(index) => {
            let matched_sentence;
            (matched_sentence, src_code) = split_off(index, src_code);

            // to update the position of the lexer
            let mut lines = matched_sentence.lines().rev();
            let column_count = lines.next().unwrap().len() + 1; // plus 1 for the last " character. or else the position will be off by one.
            let line_count = lines.count();

            let mut token_object = TokenObject::new(Token::STRING, start_position);
            token_object.update_token_value(TokenValue::String(matched_sentence));

            lexer.curr_position.move_line_by(line_count, column_count);
            lexer.add_token(token_object);

            (_, src_code) = split_off(1, src_code); // remove the " from the end of the src_code.
            return src_code;
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