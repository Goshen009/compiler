use super::{Lexer, LexerErrorTypes};
use super::tokens::*;

pub mod grouping_handlers;
use grouping_handlers::*;

pub fn check_token_syntax(next_token: &mut TokenObject, lexer: &mut Lexer) {
    // I wanted a way to get the token in front of another token, but the lexer wasn't structed for that. So right before a token is added to the list, get the token at the front of the tokens list. This is the token we'll use as the current token.
    // Get the second token from the list. This will be the previous token. And finally, the token we were about to add to the list will be the next token!

    let len = lexer.tokens.len();
    let previous_token = lexer.tokens.get(len.saturating_sub(2)).unwrap().get_token();

    let current_token = lexer.tokens.get(len.saturating_sub(1)).unwrap().get_token();
    let syntax_handler = current_token.get_syntax_check_handler();

    if syntax_handler.is_some() {
        (syntax_handler.unwrap())(previous_token, next_token, lexer);
    }

    // this is to ensure that all opened brackets are closed.
    if current_token.is_grouping_open() { grouping_open_handler(current_token, lexer.get_current_index(), lexer); }
    if current_token.is_grouping_close() { grouping_close_handler(current_token, lexer.get_current_index(), next_token, lexer); }
}

// lexer.errors.add_error(err);

pub mod struct_statement_handlers {
    use super::*;
    
    pub fn struct_syntax_check(previous_token: Token, next_token: &mut TokenObject, lexer: &mut Lexer) {
        if !previous_token.is_end_statement() /* later will check for 'public' too */ {
            lexer.errors.add_error(LexerErrorTypes::Struct_Expects_End_Statement_Behind(lexer.get_current_index()));
        }
        
        if next_token.get_token() == Token::SYMBOL {
            next_token.update_token(Token::STRUCT_NAME);  
        } else { lexer.errors.add_error(LexerErrorTypes::Struct_Expects_Symbol_Ahead(lexer.get_current_index())); }
    }

    pub fn struct_name_syntax_check(_previous_token: Token, next_token: &mut TokenObject, lexer: &mut Lexer) {
        if next_token.get_token() != Token::OPEN_CURLY {
            lexer.errors.add_error(LexerErrorTypes::Struct_Name_Expects_Open_Curly_In_Front(lexer.get_current_index()));
        }
    }
}

pub mod function_statement_handlers {
    use super::*;

    pub fn function_decl_syntax_check(previous_token: Token, next_token: &mut TokenObject, lexer: &mut Lexer) {
        if !previous_token.is_end_statement() /* later will check for 'public' too */ {
            lexer.errors.add_error(LexerErrorTypes::Function_Expects_End_Statement_Behind(lexer.get_current_index()));
        }
        
        if next_token.get_token() == Token::SYMBOL {
            next_token.update_token(Token::FUNCTION_NAME);
            
        } else { lexer.errors.add_error(LexerErrorTypes::Function_Expects_Symbol_Ahead(lexer.get_current_index())); }
    }

    pub fn function_name_syntax_check(_previous_token: Token, next_token: &mut TokenObject, lexer: &mut Lexer) {
        if next_token.get_token() != Token::OPEN_BRACKET {
            lexer.errors.add_error(LexerErrorTypes::Function_Name_Expects_Open_Bracket_In_Front(lexer.get_current_index()));
        }
    }

    pub fn function_parameter_syntax_check(_previous_token: Token, next_token: &mut TokenObject, lexer: &mut Lexer) {
        if next_token.get_token() != Token::COLON {
            lexer.errors.add_error(LexerErrorTypes::Colon_Expected(lexer.get_current_index()));
        }
    }

    pub fn function_parameter_type_syntax_check(_previous_token: Token, next_token: &mut TokenObject, lexer: &mut Lexer) {
        if next_token.get_token() != Token::CLOSE_BRACKET && next_token.get_token() != Token::COMMA {
            lexer.errors.add_error(LexerErrorTypes::Colon_Or_Close_Bracket_Expected(lexer.get_current_index()));
        }
    }
}

pub fn open_bracket_syntax_check(previous_token: Token, next_token: &mut TokenObject, lexer: &mut Lexer) {
    if previous_token == Token::FUNCTION_NAME {
        match next_token.get_token() {
            Token::CLOSE_BRACKET => (),
            Token::SYMBOL => next_token.update_token(Token::FUNCTION_PARAMETER),
            _ => lexer.errors.add_error(LexerErrorTypes::Function_Expects_Symbol_As_Its_Parameter(lexer.get_current_index() + 1)),
        }
    }
}

pub fn colon_syntax_check(previous_token: Token, next_token: &mut TokenObject, lexer: &mut Lexer) {
    if previous_token == Token::FUNCTION_PARAMETER {
        if next_token.get_token() == Token::SYMBOL {
            next_token.update_token(Token::FUNCTION_PARAMETER_TYPE);
        } else { lexer.errors.add_error(LexerErrorTypes::Expected_Type_For_Parameter(lexer.get_current_index())); }
    }
}

pub fn comma_syntax_check(previous_token: Token, next_token: &mut TokenObject, lexer: &mut Lexer) {
    if previous_token == Token::FUNCTION_PARAMETER_TYPE {
        if next_token.get_token() == Token::SYMBOL {
            next_token.update_token(Token::FUNCTION_PARAMETER);
        } else { lexer.errors.add_error(LexerErrorTypes::Function_Expects_Symbol_As_Its_Parameter(lexer.get_current_index() + 1)); }
    }
}