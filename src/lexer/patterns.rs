use super::{default_handler, symbol_or_keyword_handler, number_handler};
use super::{Token, TokenObject};

pub struct Keyword {
    pub pattern: &'static str,
    pub token: Token
}

pub const KEYWORDS: [Keyword; 6] = [
    Keyword { pattern: r"^(let)\b", token: Token::LET },
    Keyword { pattern: r"^(const)\b", token: Token::CONST },
    Keyword { pattern: r"^(return)\b", token: Token::RETURN },
    Keyword { pattern: r"^(scream)\b", token: Token::SCREAM },
    Keyword { pattern: r"^(struct)\b", token: Token::STRUCT },
    Keyword { pattern: r"^(monk)\b", token: Token::MONK },
];



pub struct Pattern {
    pub token: Token,
    pub pattern: &'static str,
    pub handler: fn (TokenObject, String) -> TokenObject,
}

// the ordering of the patterns is important, DO NOT CHANGE IT!
pub const PATTERNS: [Pattern; 34] = [
    Pattern { pattern: r"^(\r?\n)", handler: default_handler, token: Token::NEW_LINE },

    Pattern { pattern: r"^([^\S\r\n]+)", handler: default_handler, token: Token::SPACE },
    Pattern { pattern: r"^(\/\/)", handler: default_handler, token: Token::COMMENT },

    Pattern { pattern: r"^([a-zA-Z_][a-zA-Z0-9_]*)", handler: symbol_or_keyword_handler, token: Token::SYMBOL },
    Pattern { pattern: r"^([0-9]+)\b", handler: number_handler, token: Token::NUMBER },

    Pattern { pattern: r"^(\&&)", handler: default_handler, token: Token::AND },
    Pattern { pattern: r"^(\|\|)", handler: default_handler, token: Token::OR },

    Pattern { pattern: r"^(\+=)", handler: default_handler, token: Token::PLUS_ASSIGN },
    Pattern { pattern: r"^(\-=)", handler: default_handler, token: Token::MINUS_ASSIGN },
    Pattern { pattern: r"^(\->)", handler: default_handler, token: Token::DASH_GREATER },

    Pattern { pattern: r"^(>=)", handler: default_handler, token: Token::GREATER_EQUALS },
    Pattern { pattern: r"^(<=)", handler: default_handler, token: Token::LESS_EQUALS },
    Pattern { pattern: r"^(!=)", handler: default_handler, token: Token::NOT_EQUALS },
    Pattern { pattern: r"^(==)", handler: default_handler, token: Token::EQUALS },
    Pattern { pattern: r"^(>)", handler: default_handler, token: Token::GREATER },
    Pattern { pattern: r"^(<)", handler: default_handler, token: Token::LESS },

    Pattern { pattern: "^(\")", handler: default_handler, token: Token::DOUBLE_QUOTE },

    Pattern { pattern: r"^(\()", handler: default_handler, token: Token::OPEN_BRACKET },
    Pattern { pattern: r"^(\))", handler: default_handler, token: Token::CLOSE_BRACKET },
    Pattern { pattern: r"^(\{)", handler: default_handler, token: Token::OPEN_CURLY },
    Pattern { pattern: r"^(\})", handler: default_handler, token: Token::CLOSE_CURLY },
    Pattern { pattern: r"^(\[)", handler: default_handler, token: Token::OPEN_SQUARE },
    Pattern { pattern: r"^(\])", handler: default_handler, token: Token::CLOSE_SQUARE },

    Pattern { pattern: r"^(\,)", handler: default_handler, token: Token::COMMA },
    Pattern { pattern: r"^(\:)", handler: default_handler, token: Token::COLON },
    Pattern { pattern: r"^(\;)", handler: default_handler, token: Token::SEMICOLON },
    Pattern { pattern: r"^(\!)", handler: default_handler, token: Token::NOT },
    Pattern { pattern: r"^(\=)", handler: default_handler, token: Token::ASSIGN },
    Pattern { pattern: r"^(\-)", handler: default_handler, token: Token::MINUS },
    Pattern { pattern: r"^(\/)", handler: default_handler, token: Token::DIVIDE },
    Pattern { pattern: r"^(\+)", handler: default_handler, token: Token::PLUS },
    Pattern { pattern: r"^(\*)", handler: default_handler, token: Token::STAR },
    Pattern { pattern: r"^(\%)", handler: default_handler, token: Token::PERCENT },
    Pattern { pattern: r"^(\.)", handler: default_handler, token: Token::PERIOD },
];