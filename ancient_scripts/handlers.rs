use super::{Lexer, LexerErrorTypes};
use super::tokens::*;

struct SyntaxResult<'a> {
    result: bool,
    update_token: Option<Token>,
    error_type: Option<LexerErrorTypes<'a>>,
}

const fn set_update_token(update_token: Token, apply_case: Token, next_token:Token) -> Option<Token> {
    if next_token.as_u8() == apply_case.as_u8() { return Some(update_token); }
    else { return None; }
}

const fn set_update_token_for_two(update_token1: Token, apply_case1: Token, update_token2: Token, apply_case2: Token, next_token:Token) -> Option<Token> {
    if next_token.as_u8() == apply_case1.as_u8() { return Some(update_token1); }
    else if next_token.as_u8() == apply_case2.as_u8() { return Some(update_token2); }
    else { return None; }
}

pub fn check_token_syntax(next_token: &mut TokenObject, lexer: &mut Lexer) {
    let len = lexer.tokens.len();

    let previous_token = lexer.tokens.get(len.saturating_sub(2)).unwrap().get_token();
    let current_token = lexer.tokens.get(len.saturating_sub(1)).unwrap();

    let result = get_token_syntax_result(previous_token, current_token.get_token(), next_token.get_token());
    
    if result.result == true {
        let update_token = result.update_token;
        if update_token.is_some() { next_token.update_token(update_token.unwrap()); }
    } else {
        let error = result.error_type;
        if error.is_some() { lexer.errors.add_error(crate::get_error_string(error.unwrap(), current_token.get_position())); }
    }
}

const fn get_token_syntax_result<'a>(previous_token: Token, current_token: Token, next_token: Token) -> SyntaxResult<'a> {
    match current_token {
        /* STRUCTS -------------------------------------------------------------------------------- */
        Token::STRUCT => SyntaxResult { 
            result: next_token.as_u8() == Token::SYMBOL.as_u8(), 
            update_token: Some(Token::STRUCT_NAME), 
            error_type: Some(LexerErrorTypes::Expects_Name)
        },

        /* FUNCTIONS -----------------------------------------------------------------------------  */
        Token::MONK => SyntaxResult {
            result: next_token.as_u8() == Token::SYMBOL.as_u8(),
            update_token: Some(Token::FUNCTION_NAME),
            error_type: Some(LexerErrorTypes::Expects_Name)
        },

        Token::FUNCTION_NAME => SyntaxResult {
            result: next_token.as_u8() == Token::OPEN_BRACKET.as_u8(),
            update_token: Some(Token::FUNCTION_OPEN_BRACKETS),
            error_type: Some(LexerErrorTypes::Expects_Token("("))
        },

        Token::FUNCTION_PARAMETER => SyntaxResult {
            result: next_token.as_u8() == Token::COLON.as_u8(),
            update_token: None,
            error_type: Some(LexerErrorTypes::Expects_Token(":"))
        },

        Token::FUNCTION_PARAMETER_TYPE => SyntaxResult {
            result: next_token.as_u8() == Token::CLOSE_BRACKET.as_u8() || next_token.as_u8() == Token::COMMA.as_u8(),
            update_token: None,
            error_type: Some(LexerErrorTypes::Expects_Two_Tokens(",", ")"))
        },

        Token::FUNCTION_OPEN_BRACKETS => SyntaxResult {
            result: next_token.as_u8() == Token::CLOSE_BRACKET.as_u8() || next_token.as_u8() == Token::SYMBOL.as_u8(),
            update_token: set_update_token(Token::FUNCTION_PARAMETER, Token::SYMBOL, next_token),
            error_type: Some(LexerErrorTypes::Expects_Two_Tokens("parameter", ")"))
        },

        Token::DASH_GREATER => SyntaxResult {
            result: next_token.as_u8() == Token::OPEN_BRACKET.as_u8() || next_token.as_u8() == Token::SYMBOL.as_u8(),
            update_token: set_update_token_for_two(Token::FUNCTION_RETURN_OPEN_BRACKETS, Token::OPEN_BRACKET, Token::FUNCTION_RETURN_TYPE, Token::SYMBOL, next_token),
            error_type: Some(LexerErrorTypes::Expects_Type)
        },

        Token::FUNCTION_RETURN_OPEN_BRACKETS => SyntaxResult {
            result: next_token.as_u8() == Token::SYMBOL.as_u8(),
            update_token: Some(Token::FUNCTION_RETURN_TYPE),
            error_type: Some(LexerErrorTypes::Expects_Type)
        },

        Token::FUNCTION_RETURN_TYPE => SyntaxResult {
            result: next_token.as_u8() == Token::CLOSE_BRACKET.as_u8() || next_token.as_u8() == Token::COMMA.as_u8(),
            update_token: None,
            error_type: Some(LexerErrorTypes::Expects_Two_Tokens(",", ")"))
        },
        
        /* MISC ----------------------------------------------------------------------------------- */
        Token::COLON => {
            if previous_token.as_u8() == Token::FUNCTION_PARAMETER.as_u8() {
                SyntaxResult {
                    result: next_token.as_u8() == Token::SYMBOL.as_u8(),
                    update_token: Some(Token::FUNCTION_PARAMETER_TYPE),
                    error_type: Some(LexerErrorTypes::Expects_Type)
                }
            } else {
                SyntaxResult {
                    result: false,
                    update_token: None,
                    error_type: Some(LexerErrorTypes::Unexpected_Token(":"))
                }
            }
        }

        Token::COMMA => {
            // try and set only the update token based on the responses given.
            
            if previous_token.as_u8() == Token::FUNCTION_PARAMETER_TYPE.as_u8() {
                SyntaxResult {
                    result: next_token.as_u8() == Token::SYMBOL.as_u8(),
                    update_token: Some(Token::FUNCTION_PARAMETER),
                    error_type: Some(LexerErrorTypes::Expects_Token("parameter"))
                }
            } else {
                SyntaxResult {
                    result: false,
                    update_token: None,
                    error_type: Some(LexerErrorTypes::Unexpected_Token(","))
                }
            }
        }

        Token::CLOSE_BRACKET => {
            if previous_token.as_u8() == Token::FUNCTION_OPEN_BRACKETS.as_u8() || previous_token.as_u8() == Token::FUNCTION_PARAMETER_TYPE.as_u8() {                SyntaxResult {
                    result: next_token.as_u8() == Token::OPEN_CURLY.as_u8() || next_token.as_u8() == Token::DASH_GREATER.as_u8(),
                    update_token: set_update_token(Token::FUNCTION_OPEN_CURLY, Token::OPEN_CURLY, next_token),
                    error_type: Some(LexerErrorTypes::Expects_Two_Tokens("->", "{"))
                }
            }  else {
                SyntaxResult {
                    result: false,
                    update_token: None,
                    error_type: Some(LexerErrorTypes::Unexpected_Token(")"))
                }
            }
        }

        _ => SyntaxResult {
            result: true,
            update_token: None,
            error_type: None
        }
    }
}