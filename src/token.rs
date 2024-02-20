type TokenKind = String;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: &str) -> Self {
        Token {
            kind: kind.to_string(),
            literal: kind.to_string(),
        }
    }
    pub fn with(kind: &str, literal: &str) -> Self {
        Token {
            kind: kind.to_string(),
            literal: literal.to_string(),
        }
    }
}

macro_rules! add_token {
    ($x:ident) => {
        pub const $x: &str = stringify!($x);
    };
    ($x:ident,$y:expr) => {
        pub const $x: &str = $y;
    };
}

// Operators
add_token!(ASSIGN, "=");
add_token!(PLUS, "+");
add_token!(MINUS, "-");
add_token!(PRODUCT, "*");
add_token!(DIV, "/");
add_token!(MOD, "%");
add_token!(BANG, "!");

// Compare
add_token!(LT, "<");
add_token!(LT_OR_EQ, "<=");
add_token!(GT, ">");
add_token!(GT_OR_EQ, ">=");
add_token!(EQ, "==");
add_token!(NOT_EQ, "!=");

// Boolean And, Or
add_token!(AND, "&&");
add_token!(OR, "||");
// Bit And, Or
add_token!(BIT_AND, "&");
add_token!(BIT_OR, "|");

add_token!(COMMA, ",");
add_token!(SEMICOLON, ";");

add_token!(LPAREN, "(");
add_token!(RPAREN, ")");
add_token!(LBRACE, "{");
add_token!(RBRACE, "}");

// keywords
add_token!(ILLEGAL);
add_token!(EOF);
add_token!(IDENT);
add_token!(FUNCTION);
add_token!(LET);
add_token!(TRUE);
add_token!(FALSE);
add_token!(IF);
add_token!(ELSE);
add_token!(RETURN);
add_token!(INT);

pub fn get_token_kind(word: &str) -> TokenKind {
    match word {
        "fn" => String::from(FUNCTION),
        "let" => String::from(LET),
        "true" => String::from(TRUE),
        "false" => String::from(FALSE),
        "if" => String::from(IF),
        "else" => String::from(ELSE),
        "return" => String::from(RETURN),
        &_ => String::from(IDENT),
    }
}
