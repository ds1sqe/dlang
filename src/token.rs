#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    // Operators
    Assign,
    Plus,
    Minus,
    Product,
    Divide,
    Mod,
    Bang,

    // Compare
    LT,
    LT_OR_EQ,
    GT,
    GT_OR_EQ,
    EQ,
    NOT_EQ,

    // Boolean And, Or
    And,
    Or,
    // Bit And, Or
    Bit_And,
    Bit_Or,

    Comma,
    Semicolon,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // keywords
    Illegal,
    EOF,
    Ident,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    Int,
    String,
}

impl Kind {
    pub fn to_str(&self) -> &str {
        match self {
            Kind::Assign => "=",
            Kind::Plus => "+",
            Kind::Minus => "-",
            Kind::Product => "*",
            Kind::Divide => "/",
            Kind::Mod => "%",
            Kind::Bang => "!",

            // Compare
            Kind::LT => "<",
            Kind::LT_OR_EQ => "<=",
            Kind::GT => ">",
            Kind::GT_OR_EQ => ">=",
            Kind::EQ => "==",
            Kind::NOT_EQ => "!=",

            // Boolean And, Or
            Kind::And => "&&",
            Kind::Or => "||",
            // Bit And, Or
            Kind::Bit_And => "&",
            Kind::Bit_Or => "|",

            Kind::Comma => ",",
            Kind::Semicolon => ";",

            Kind::LPAREN => "(",
            Kind::RPAREN => ")",
            Kind::LBRACE => "{",
            Kind::RBRACE => "}",
            // keywords
            Kind::Illegal => "Illegal",
            Kind::EOF => "EOF",
            Kind::Function => "fn",
            Kind::Ident => "Ident",
            Kind::Let => "let",
            Kind::True => "true",
            Kind::False => "false",
            Kind::If => "if",
            Kind::Else => "else",
            Kind::Return => "return",
            Kind::Int => "Int",
            Kind::String => "String",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: Kind) -> Self {
        Token {
            kind,
            literal: kind.to_str().to_string(),
        }
    }
    pub fn with(kind: Kind, literal: &str) -> Self {
        Token {
            kind,
            literal: literal.to_string(),
        }
    }
}

pub fn get_token_kind(word: &str) -> Kind {
    match word {
        "fn" => Kind::Function,
        "let" => Kind::Let,
        "true" => Kind::True,
        "false" => Kind::False,
        "if" => Kind::If,
        "else" => Kind::Else,
        "return" => Kind::Return,
        &_ => Kind::Ident,
    }
}
