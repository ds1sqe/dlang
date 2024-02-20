#![allow(dead_code)]

use crate::token;
use crate::token::Token;
pub mod error;

pub struct Lexer {
    input: String,
    pos: usize,
    next_pos: usize,
    cur: char,
}

impl Lexer {
    /// create new lexer from given input.
    ///
    /// * `input`: [String]
    pub fn new(input: String) -> Self {
        let mut lex = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            cur: '\0',
        };
        lex.read_char();
        lex
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.cur = '\0'
        } else {
            self.cur = self.input.chars().nth(self.next_pos).unwrap()
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    fn peek_char(&self) -> char {
        if self.next_pos >= self.input.len() {
            return '\0';
        } else {
            return self.input.chars().nth(self.next_pos).unwrap();
        }
    }

    fn skip_whitespace(&mut self) {
        while self.cur.is_whitespace() {
            self.read_char()
        }
    }

    fn read_id(&mut self) -> String {
        let start = self.pos;
        while !self.peek_char().is_whitespace() && self.peek_char().is_alphanumeric() {
            self.read_char();
        }
        self.input[start..self.pos + 1].to_string()
    }

    fn read_num(&mut self) -> Result<String, error::LexerError> {
        let start = self.pos;
        while !self.peek_char().is_whitespace() && self.peek_char().is_alphanumeric() {
            if self.peek_char().is_alphabetic() {
                let err = error::LexerError {
                    pos_start: start,
                    pos_end: self.pos,
                    reason: self.input[start..self.pos + 1].to_string() + " is not a numeric",
                };
                return Err(err);
            }
            self.read_char();
        }
        Ok(self.input[start..self.pos + 1].to_string())
    }

    /// get next token
    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        let mut token: Token = Token::new(token::ILLEGAL);

        match self.cur {
            '+' => token = Token::new(token::PLUS),
            '-' => token = Token::new(token::MINUS),
            '*' => token = Token::new(token::PRODUCT),
            '/' => token = Token::new(token::DIV),
            '%' => token = Token::new(token::MOD),
            '(' => token = Token::new(token::LPAREN),
            ')' => token = Token::new(token::RPAREN),
            '{' => token = Token::new(token::LBRACE),
            '}' => token = Token::new(token::RBRACE),
            ',' => token = Token::new(token::COMMA),
            ';' => token = Token::new(token::SEMICOLON),
            '\0' => token = Token::new(token::EOF),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token::new(token::EQ);
                } else {
                    token = Token::new(token::ASSIGN);
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token::new(token::NOT_EQ);
                } else {
                    token = Token::new(token::BANG);
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token::new(token::LT_OR_EQ);
                } else {
                    token = Token::new(token::LT);
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token::new(token::GT_OR_EQ);
                } else {
                    token = Token::new(token::GT);
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    self.read_char();
                    token = Token::new(token::AND);
                } else {
                    token = Token::new(token::BIT_AND);
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    self.read_char();
                    token = Token::new(token::OR);
                } else {
                    token = Token::new(token::BIT_OR);
                }
            }
            any => {
                if any.is_alphabetic() {
                    token.literal = self.read_id();
                    token.kind = token::get_token_kind(&token.literal);
                } else if any.is_numeric() {
                    let read_result = self.read_num();
                    if read_result.is_ok() {
                        token.literal = read_result.unwrap();
                        token.kind = token::INT.to_string();
                    } else {
                        token.literal = read_result.err().unwrap().reason;
                    }
                }
            }
        }
        self.read_char();
        token
    }
}
