type TokenKind = String;

#[allow(dead_code)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}

macro_rules! add_token {
    ($x:ident) => {
        pub const $x: &str = stringify!($x);
    };
    ($x:ident,$y:expr) => {
        pub const $x: &str = $y;
    };
}

add_token!(ILLEGAL);
add_token!(EOF);
add_token!(IDENT);

// Operators
add_token!(ASSIGN, "=");
add_token!(PLUS, "+");
add_token!(MINUS, "-");
add_token!(PROD, "*");
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
add_token!(BIT_AND, "&");
add_token!(BIT_OR, "|");

// Boolean And, Or
add_token!(AND, "&&");
add_token!(OR, "||");

add_token!(COMMA, ",");
add_token!(SEMICOLON, ";");

add_token!(LPAREN, "(");
add_token!(RPAREN, ")");
add_token!(LBRACE, "{");
add_token!(RBRACE, "}");

// keywords
add_token!(FUNCTION);
add_token!(LET);
add_token!(TRUE);
add_token!(FALSE);
add_token!(IF);
add_token!(ELSE);
add_token!(RETURN);

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
