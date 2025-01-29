use super::Position;

#[derive(Clone)]
pub struct TokenObject {
    token: Token,
    value: Option<TokenValue>, // i'm using option because i want to move out of it
    position: Position,
}

impl TokenObject {
    pub fn new(token: Token, position: Position) -> Self {
        Self {
            token,
            value: Some(TokenValue::None),
            position,
        }
    }

    pub fn print_self(&mut self) { // moves out of token value, does the checks and moves back in.
        let token_value = self.take_value();
        match &token_value {
            TokenValue::None => println!("{:<35} {}", format!("{:?}", self.get_token()), self.get_position()),
            TokenValue::Number(val) => println!("{:<35} {}", format!("{:?}({})", self.get_token(), val), self.get_position()),
            TokenValue::String(val) => println!("{:<35} {}", format!("{:?}({})", self.get_token(), val), self.get_position()),
        }

        self.update_token_value(token_value);
    }

    pub fn update_token_value(&mut self, value: TokenValue) {
        self.value = Some(value);
    }

    pub fn update_token(&mut self, token: Token) {
        self.token = token;
    }

    pub fn get_token(&self) -> Token {
        self.token
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn take_value(&mut self) -> TokenValue {
        self.value.take().unwrap()
    }
}

#[derive(Clone)]
pub enum TokenValue {
    String(String),
    Number(i64),
    None,
}

impl TokenValue {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            TokenValue::String(val) => Some(val),
            _ => None
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    STRING,         // string
    SYMBOL,         // var_names
    NUMBER,         // integer number

    FUNCTION_NAME,
    FUNCTION_PARAMETER,
    FUNCTION_OPEN_CURLY,
    FUNCTION_RETURN_TYPE,
    FUNCTION_OPEN_BRACKETS,
    FUNCTION_PARAMETER_TYPE,
    FUNCTION_RETURN_OPEN_BRACKETS,

    STRUCT_NAME,
    STRUCT_FIELD,

    PLUS_ASSIGN,    // +=
    MINUS_ASSIGN,   // -=

    DASH_GREATER,   // ->

    AND,            // &&
    OR,             // ||
    
    GREATER_EQUALS, // >=
    LESS_EQUALS,    // <=
    NOT_EQUALS,     // !=
    EQUALS,         // ==
    GREATER,        // >
    LESS,           // <
    
    OPEN_BRACKET,   // (
    CLOSE_BRACKET,  // )
    OPEN_CURLY,     // {
    CLOSE_CURLY,    // }
    OPEN_SQUARE,    // [
    CLOSE_SQUARE,   // ]

    DOUBLE_QUOTE,   // "

    COMMA,          // ,
    COLON,          // :
    SEMICOLON,      // ;
    NOT,            // !
    MINUS,          // -
    DIVIDE,         // /
    ASSIGN,         // =
    PLUS,           // +
    STAR,           // *
    PERCENT,        // %
    PERIOD,         // .
    
    LET,
    CONST,
    RETURN,
    SCREAM,
    STRUCT,
    MONK,
    IF,
    ELSE,

    START,
    SPACE,
    COMMENT,
    NEW_LINE,
    ERROR,
    EOF,
}

type TokenSyntaxHandler = fn(Token, &mut TokenObject, &mut super::Lexer);

impl Token {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn get_syntax_check_handler(&self) -> Option<TokenSyntaxHandler> {
        match *self {
            Token::START | Token::ERROR => None,

            // Token::STRUCT => Some(struct_statement_handlers::struct_syntax_check),
            // Token::STRUCT_NAME => Some(struct_statement_handlers::struct_name_syntax_check),

            // Token::MONK => Some(function_statement_handlers::function_decl_syntax_check),
            // Token::FUNCTION_NAME => Some(function_statement_handlers::function_name_syntax_check),
            // Token::FUNCTION_PARAMETER => Some(function_statement_handlers::function_parameter_syntax_check),
            // Token::FUNCTION_PARAMETER_TYPE => Some(function_statement_handlers::function_parameter_type_syntax_check),

            // Token::OPEN_CURLY => Some(open_curly_syntax_check),
            // Token::OPEN_BRACKET => Some(open_bracket_syntax_check),

            // Token::COLON => Some(colon_syntax_check),
            // Token::COMMA => Some(comma_syntax_check),

            _ => None,
        }
    }

    pub const fn is_grouping_open(&self) -> bool {
        match self {
            Token::OPEN_BRACKET | Token:: OPEN_CURLY | Token::OPEN_SQUARE | Token::FUNCTION_OPEN_BRACKETS => true,
            _ => false
        }
    }

    pub const fn is_grouping_close(&self) -> bool {
        match self {
            Token::CLOSE_BRACKET | Token::CLOSE_CURLY | Token::CLOSE_SQUARE => true,
            _ => false
        }
    }

    pub const fn can_start_new_statement(&self) -> bool {
        match self {
            Token::START | Token::OPEN_CURLY | Token::CLOSE_CURLY | Token::SEMICOLON => true,
            _ => false
        }
    }

    // pub const fn is_start_statementss(&self) -> bool {
    //     match self {
    //         Token::LET | Token::SYMBOL | Token::STRUCT | Token::MONK => true,
    //         Token::CONST | Token::RETURN | Token::SCREAM | Token::IF | Token::ELSE => true,
    //         _ => false
    //     }
    // }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::OPEN_BRACKET => write!(f, "("),
            Token::OPEN_CURLY => write!(f, "{{"),
            Token::OPEN_SQUARE => write!(f, "["),

            Token::CLOSE_BRACKET => write!(f, ")"),
            Token::CLOSE_CURLY => write!(f, "}}"),
            Token::CLOSE_SQUARE => write!(f, "]"),

            Token::DOUBLE_QUOTE => write!(f, "\""),

            _ => write!(f, ""),
        }
    }
}