use super::*;

#[allow(non_snake_case)]
pub fn check_grouping_at_EOF(lexer: &mut Lexer) {
    // this will pop out all the items from the grouping indexes on the error. If there's any index that remains in the index list, then it means that bracket was not closed.
    while let Some(index) = lexer.errors.open_bracket_indexes.pop() {
        lexer.errors.add_error(LexerErrorTypes::Opened_Grouping_That_Was_Never_Closed(index));
    }

    while let Some(index) = lexer.errors.open_curly_indexes.pop() {
        lexer.errors.add_error(LexerErrorTypes::Opened_Grouping_That_Was_Never_Closed(index));
    }

    while let Some(index) = lexer.errors.open_square_indexes.pop() {
        lexer.errors.add_error(LexerErrorTypes::Opened_Grouping_That_Was_Never_Closed(index));
    }
}

pub fn grouping_open_handler(current_token: Token, index: usize, lexer: &mut Lexer) {
    match current_token { // there's a list of indexes and everytime we enter a grouping token, we add to that list.
        Token::OPEN_BRACKET => lexer.errors.open_bracket_indexes.push(index),
        Token::OPEN_CURLY => lexer.errors.open_curly_indexes.push(index),
        Token::OPEN_SQUARE => lexer.errors.open_square_indexes.push(index),
        _ => (),
    }
}

pub fn grouping_close_handler(current_token: Token, token_index: usize, next_token: &mut TokenObject, lexer: &mut Lexer) {
    match current_token { // we'll pop from the list of indexes when we leave a grouping token. If the value of the pop is None, then we know there's an error.
        Token::CLOSE_BRACKET => close_bracket_syntax_check(lexer.errors.open_bracket_indexes.pop(), token_index, next_token, lexer,),
        Token::CLOSE_CURLY => {
            if lexer.errors.open_curly_indexes.pop().is_none() {
                let error = LexerErrorTypes::Closed_Grouping_That_Was_Never_Opened(token_index);
                lexer.errors.add_error(error);
            }
        },
        Token::CLOSE_SQUARE => {
            if lexer.errors.open_square_indexes.pop().is_none() {
                let error = LexerErrorTypes::Closed_Grouping_That_Was_Never_Opened(token_index);
                lexer.errors.add_error(error);
            }
        },
        _ => (),
    }
}

pub fn close_bracket_syntax_check(open_bracket_index: Option<usize>, token_index: usize, next_token: &mut TokenObject, lexer: &mut Lexer) {
    if open_bracket_index.is_none() {
        lexer.errors.add_error(LexerErrorTypes::Closed_Grouping_That_Was_Never_Opened(token_index));
        return;
    }

    // to know what can/should come after the bracket, we need to know what type of bracket it is.
    let token_before_open_bracket = lexer.get_token_at(&open_bracket_index.unwrap().saturating_sub(1)).get_token();
    
    if token_before_open_bracket == Token::FUNCTION_NAME {
        if next_token.get_token() != Token::OPEN_CURLY /* | next_token != Token::DASH_GREATER */ {
            lexer.errors.add_error(LexerErrorTypes::Open_Curly_Expected(token_index));
            return;
        }
    }
}