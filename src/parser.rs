use crate::{
    ast::{LetStatement, Program, Statement},
    lexer::Lexer,
    token::Token,
};

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    next_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut lexer = lexer;
        let cur_token = lexer.next();
        let next_token = lexer.next();
        Self {
            lexer,
            cur_token,
            next_token,
        }
    }

    fn next(&mut self) {
        self.cur_token = self.next_token.clone();
        self.next_token = self.lexer.next()
    }

    pub fn parse(&mut self) -> Program {
        let mut program = Program::new();

        program
    }

    fn parse_statement(&mut self) -> Box<dyn Statement> {
        match self.cur_token.kind {}
    }
    fn parse_let_statement(&mut self) -> LetStatement {
        unimplemented!()
    }
}
