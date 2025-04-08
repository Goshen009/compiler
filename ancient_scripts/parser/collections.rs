use std::{collections::HashMap, sync::LazyLock};
use super::{Token, handler_lookup::*};

pub static MAP: LazyLock<HashMap<Token, (BindingPower, Option<NudHandler>, Option<LedHandler>, Option<StmtHandler>)>> = LazyLock::new(|| {
    HashMap::from([
        // Logical
        (Token::AND, (BindingPower::Logical, None, get_parse_binary_expression(), None)),
        (Token::OR, (BindingPower::Logical, None, get_parse_binary_expression(), None)),

        // Relational
        (Token::LESS, (BindingPower::Relational, None, get_parse_binary_expression(), None)),
        (Token::LESS_EQUALS, (BindingPower::Relational, None, get_parse_binary_expression(), None)),
        (Token::GREATER, (BindingPower::Relational, None, get_parse_binary_expression(), None)),
        (Token::GREATER_EQUALS, (BindingPower::Relational, None, get_parse_binary_expression(), None)),
        (Token::EQUALS, (BindingPower::Relational, None, get_parse_binary_expression(), None)),
        (Token::NOT_EQUALS, (BindingPower::Relational, None, get_parse_binary_expression(), None)),

        // Additive & Multiplictive
        (Token::PLUS, (BindingPower::Additive, None, get_parse_binary_expression(), None)),
        (Token::DASH, (BindingPower::Additive, None, get_parse_binary_expression(), None)),

        (Token::STAR, (BindingPower::Multiplicative, None, get_parse_binary_expression(), None)),
        (Token::SLASH, (BindingPower::Multiplicative, None, get_parse_binary_expression(), None)),
        (Token::PERCENT, (BindingPower::Multiplicative, None, get_parse_binary_expression(), None)),

        // Literals & Symbols
        (Token::NUMBER, (BindingPower::Primary, get_parse_primary_expression(), None, None)),
        (Token::STRING, (BindingPower::Primary, get_parse_primary_expression(), None, None)),
        (Token::IDENTIFIER, (BindingPower::Primary, get_parse_primary_expression(), None, None)),
    ])
});

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingPower {
    /* TO CHECK IF ALL TOKENS ARE BOUND TO A BINDING POWER, */
    /* I CAN RUN THE LOOP FOR THE KEYWORDS AND PATTERNS IN THE LEXER */
    /* AND CHECK IF THE PRECEDENCE HASHMAP HAS THAT TOKEN IN IT'S TABLE */

    DefaultBp = 0,
    Comma,
    Assignment,
    Logical,
    Relational,
    Additive,
    Multiplicative,
    Unary,
    Call,
    Member,
    Primary,
}