use crate::lexer::tokens::TokenValue;

use super::{Token, parser_obj::Parser, expr_stmt::*, collections::*};

pub type StmtHandler = fn(&mut Parser) -> StmtGeneral;
pub type NudHandler = fn(&mut Parser) -> ExprGeneral;
pub type LedHandler = fn(&mut Parser, ExprGeneral, BindingPower) -> ExprGeneral;

pub fn parse_statement(parser: &mut Parser) -> StmtGeneral {
    let stmt_fn = stmt_lookup(parser.get_current_token());

    if let Some(func) = stmt_fn {
        return func(parser)
    } else {
        let expression = parse_expr(parser, BindingPower::DefaultBp);
        parser.expect(Token::SEMICOLON);

        return Box::new(
            ExpressionStatement {
                expression
            }
        )
    }
}

fn parse_primary_expression(parser: &mut Parser) -> ExprGeneral {
    let current_token = parser.advance();
    let mut num_value = 0;
    let mut string_value = String::new();

    match current_token.get_token_value() {
        TokenValue::Number(val) => {
            num_value = *val;
        }
        TokenValue::String(val) => {
            string_value = val.clone();
        }
        TokenValue::None => {}
    }

    match current_token.get_token() {
        Token::NUMBER => {
            Box::new(
                NumberExpression {
                    value: num_value
                }
            )
        },
        Token::STRING => {
            Box::new(
                StringExpression {
                    value: string_value
                }
            )
        }
        Token::IDENTIFIER => {
            Box::new(
                SymbolExpression {
                    value: string_value
                }
            )
        }
        _ => {
            panic!("Help me Smeeee");
        }
    }
}

fn parse_binary_expression(parser: &mut Parser, left: ExprGeneral, bp: BindingPower) -> ExprGeneral {   
    let operator = parser.advance().get_token();
    let right = parse_expr(parser, bp);

    Box::new(
        BinaryExpression {
            left,
            operator,
            right
        }
    )
}

// 40:34 is where i was kinda confused
fn parse_expr(parser: &mut Parser, bp: BindingPower) -> ExprGeneral {
    let current_token = parser.get_current_token();
    let nud_fn = nud_lookup(current_token);

    // the panic for nud_fn should happen here!
    
    let mut left = nud_fn(parser);

    while bp_lookup(parser.get_current_token()) as i8 > bp as i8 {
        let current_token = parser.get_current_token();
        let led_fn = led_lookup(current_token);

        left = led_fn(parser, left, bp);
    }

    left
}


pub fn get_parse_statement() -> Option<StmtHandler> {
    Some(parse_statement as StmtHandler)
}

pub fn get_parse_binary_expression() -> Option<LedHandler> {
    Some(parse_binary_expression as LedHandler)
}

pub fn get_parse_primary_expression() -> Option<NudHandler> {
    Some(parse_primary_expression as NudHandler)
}


pub fn bp_lookup(token: Token) -> BindingPower {
    if let Some(result) = MAP.get(&token) {
        let &(bp, _, _, _) = result;
        return bp;
    }

    BindingPower::DefaultBp
    // panic!("No such binding power for token found");
}

pub fn nud_lookup(token: Token) -> NudHandler {
    if let Some(result) = MAP.get(&token) {
        let &(_, nud_handler, _, _) = result;
        if let Some(handler) = nud_handler {
            return handler;
        }
    }

    panic!("No such binding power for token found");
}

pub fn led_lookup(token: Token) -> LedHandler {
    if let Some(result) = MAP.get(&token) {
        let &(_, _, led_handler, _) = result;
        if let Some(handler) = led_handler {
            return handler;
        }
    }

    panic!("No such binding power for token found");
}

pub fn stmt_lookup(token: Token) -> Option<StmtHandler> {
    if let Some(result) = MAP.get(&token) {
        let &(_, _, _, stmt_handler) = result;
        if let Some(handler) = stmt_handler {
            return Some(handler);
        }
    }

    None

    // panic!("No such binding power for token found");
}