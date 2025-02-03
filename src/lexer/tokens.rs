use super::Position;

#[derive(Clone)]
pub struct TokenObject {
    token: Token,
    position: Position,
    value: Option<TokenValue>, // i'm using option because i want to move out of it
}

impl TokenObject {
    pub fn new(token: Token, position: Position) -> Self {
        Self {
            token,
            position,
            value: Some(TokenValue::None),
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

    pub fn get_token(&self) -> Token {
        self.token
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn take_value(&mut self) -> TokenValue {
        let curr_value = self.value.take().unwrap();
        self.update_token_value(TokenValue::None); // just to make sure that Token Value is never None

        curr_value
    }
}

#[derive(Clone)]
pub enum TokenValue {
    String(String),
    Number(i32),
    None,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    STRING,         // string
    SYMBOL,         // var_names
    NUMBER,         // integer number

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

impl Token {
    pub fn as_expect_error(&self) -> String {
        match *self {
            Token::OPEN_CURLY => format!("Expects '{{'"),
            Token::CLOSE_CURLY => format!("Expects '}}'"),
            Token::OPEN_BRACKET => format!("Expects '('"),
            Token::CLOSE_BRACKET => format!("Expects ')'"),
            Token::CLOSE_SQUARE => format!("Expects ']'"),
            Token::COLON => format!("Expects ':'"),
            Token::SEMICOLON => format!("Expects ';'"),
            Token::COMMA => format!("Expects ','"),
            Token::ASSIGN => format!("Expects '='"),
            _ => format!("")
        }
    }
}

// impl std::fmt::Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Token::OPEN_BRACKET => write!(f, "("),
//             Token::OPEN_CURLY => write!(f, "{{"),
//             Token::OPEN_SQUARE => write!(f, "["),

//             Token::CLOSE_BRACKET => write!(f, ")"),
//             Token::CLOSE_CURLY => write!(f, "}}"),
//             Token::CLOSE_SQUARE => write!(f, "]"),

//             Token::DOUBLE_QUOTE => write!(f, "\""),

//             _ => write!(f, ""),
//         }
//     }
// }