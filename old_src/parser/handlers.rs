use super::*;

pub type StmtHandler = fn(&mut Parser) -> Option<StatementEnum>;
pub type NudHandler = fn(&mut Parser) -> Option<ExpressionEnum>;
pub type LedHandler = fn(&mut Parser, ExpressionEnum, BindingPower) -> Option<ExpressionEnum>;
pub type TypeNudHandler = fn(&mut Parser) -> Option<TypesEnum>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum BindingPower {
    None,
    DefaultBp,
    Assignment,
    // Logical,
    // Relational,
    Additive,
    Multiplicative,
    // Unary,
    // Call,
    // Member,
    // Primary,
}

impl Token {
    pub const fn binding_power(&self) -> BindingPower {
        match *self {
            Token::ASSIGN | Token::PLUS_ASSIGN | Token::MINUS_ASSIGN => BindingPower::Assignment,

            Token::STAR | Token::DIVIDE | Token::PERCENT => BindingPower::Multiplicative,
            Token::PLUS | Token::MINUS => BindingPower::Additive,

            // Token::LESS | Token::LESS_EQUALS => BindingPower::Relational,
            // Token::GREATER | Token::GREATER_EQUALS => BindingPower::Relational,
            // Token::EQUALS | Token::NOT_EQUALS => BindingPower::Relational,

            // Token::AND | Token::OR => BindingPower::Logical,

            _=> BindingPower::None
        }
    }

    pub const fn nud(&self) -> Option<NudHandler> {
        match *self {
            Token::NUMBER | Token::STRING | Token::SYMBOL => Some(parse_primary_expression),
            Token::OPEN_BRACKET => Some(parse_bracket_expression),
            Token::OPEN_SQUARE => Some(parse_array_expression),
            Token::MINUS | Token::PLUS => Some(parse_prefix_expression),
            _ => None,
        }
    }

    pub const fn led(&self) -> Option<LedHandler> {
        match self.binding_power() {
            /* BindingPower::Logical | BindingPower::Relational | */ BindingPower::Additive | BindingPower::Multiplicative => Some(parse_binary_expression),
            // BindingPower::Assignment => Some(parse_assignment_expression),
            _ => None,
        }
    }

    pub const fn stmt(&self) -> Option<StmtHandler> {
        match self {
            Token::LET | Token::CONST => Some(parse_variable_declaration_statement),
            Token::STRUCT => Some(parse_struct_declaration_statement),
            Token::MONK => Some(parse_function_declaration_statement),
            _ => None,
        }
    }

    pub const fn nud_type(&self) -> Option<TypeNudHandler> {
        match *self {
            Token::SYMBOL => Some(parse_primary_type),
            Token::OPEN_SQUARE => Some(parse_array_type),
            Token::OPEN_BRACKET => Some(parse_tuple_type),
            _ => None,
        }
    }
}