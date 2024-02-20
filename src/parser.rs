use crate::{
    ast::{
        BlockStatement, BooleanLiteral, CallExpression, Expression, ExpressionStatement,
        FunctionLiteral, Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement,
        PrefixExpression, Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::Kind,
    token::Token,
};

pub mod errors;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Or,
    And,
    BitOr,
    BitAnd,
    Equals,  // ==
    Cmp,     // > or <
    Sum,     // +
    Product, // *
    Prefix,  // -(val) or !(val)
    Call,    // calling function like func(val)
}

fn find_precedences(kind: Kind) -> Precedence {
    match kind {
        Kind::Or => Precedence::Or,
        Kind::And => Precedence::And,
        Kind::Bit_Or => Precedence::BitOr,
        Kind::Bit_And => Precedence::BitAnd,
        Kind::EQ | Kind::NOT_EQ => Precedence::Equals,
        Kind::LT | Kind::LT_OR_EQ | Kind::GT | Kind::GT_OR_EQ => Precedence::Cmp,
        Kind::Plus | Kind::Minus => Precedence::Sum,
        Kind::Product | Kind::Divide | Kind::Mod => Precedence::Product,
        Kind::LPAREN => Precedence::Call,
        __ => Precedence::Lowest,
    }
}

fn is_infix(kind: &Kind) -> bool {
    match kind {
        Kind::Plus
        | Kind::Minus
        | Kind::Divide
        | Kind::Product
        | Kind::EQ
        | Kind::NOT_EQ
        | Kind::LT
        | Kind::LT_OR_EQ
        | Kind::GT
        | Kind::GT_OR_EQ
        | Kind::Bit_Or
        | Kind::Bit_And
        | Kind::Or
        | Kind::And
        | Kind::LPAREN => true,
        __ => false,
    }
}

pub struct Parser {
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

    fn peek_next(&self) -> &Token {
        &self.next_token
    }
    fn peek_next_is(&self, kind: &Kind) -> bool {
        &self.next_token.kind == kind
    }
    fn expect_next_is(&mut self, kind: &Kind) -> bool {
        if (&self.next_token.kind == kind) {
            self.next();
            true
        } else {
            false
        }
    }

    fn cur_precedence(&self) -> Precedence {
        find_precedences(self.cur_token.kind)
    }
    fn peek_precedence(&self) -> Precedence {
        find_precedences(self.next_token.kind)
    }
    pub fn parse(&mut self) -> Program {
        let mut program = Program::new();

        while self.cur_token.kind != Kind::EOF {
            let cur_stm = self.parse_statement();
            if cur_stm.is_ok() {
                program.push_stm(cur_stm.ok().unwrap());
            }
            self.next()
        }

        program
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, errors::ParseError> {
        match self.cur_token.kind {
            Kind::Let => {
                let res = self.parse_let_statement();
                if res.is_ok() {
                    return Ok(Box::new(res.ok().unwrap()));
                } else {
                    return Err(errors::ParseError {});
                }
            }
            Kind::Return => {
                return Ok(Box::new(self.parse_return_statement()));
            }
            __ => Ok(Box::new(self.parse_expression_statement())),
        }
    }
    fn parse_let_statement(&mut self) -> Result<LetStatement, errors::ParseError> {
        let cur_token = self.cur_token.clone();

        if !self.peek_next_is(&Kind::Ident) {
            return Err(errors::ParseError {});
        }

        self.next();

        let mut stm = LetStatement {
            token: cur_token,
            identifier: Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            },
            value: None,
        };
        if self.peek_next().kind == Kind::Assign {
            self.next(); // cur_token will be = (assign)
            self.next(); // cur_token will be rightside of =
            stm.value = self.parse_expression(Precedence::Lowest).ok()
        }
        if self.peek_next_is(&Kind::Semicolon) {
            self.next();
        }
        return Ok(stm);
    }
    fn parse_return_statement(&mut self) -> ReturnStatement {
        let mut stm = ReturnStatement {
            token: self.cur_token.clone(),
            value: None,
        };

        if self.peek_next().kind != Kind::Semicolon {
            self.next();
            stm.value = self.parse_expression(Precedence::Lowest).ok();
        }
        return stm;
    }
    fn parse_expression_statement(&mut self) -> ExpressionStatement {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest).ok();

        ExpressionStatement { token, expression }
    }

    fn parse_identifier(&mut self) -> Identifier {
        Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }
    }

    fn parse_integer_literal(&mut self) -> Result<IntegerLiteral, errors::IntegerParseError> {
        let value = self.cur_token.literal.parse();
        if value.is_err() {
            return Err(errors::IntegerParseError {});
        }
        let value = value.unwrap();
        Ok(IntegerLiteral {
            token: self.cur_token.clone(),
            value,
        })
    }

    fn parse_bool_literal(&mut self) -> BooleanLiteral {
        let value = self.cur_token.literal.parse().unwrap();

        BooleanLiteral {
            token: self.cur_token.clone(),
            value,
        }
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, errors::ParseError> {
        let token = self.cur_token.clone();
        let mut statements = Vec::new();

        self.next();

        while self.cur_token.kind != Kind::RBRACE && self.cur_token.kind != Kind::EOF {
            let stm = self.parse_statement();
            if stm.is_err() {
                return Err(errors::ParseError {});
            }
            statements.push(stm.ok().unwrap());
            self.next();
        }
        return Ok(BlockStatement { token, statements });
    }

    fn parse_prefix_expression(&mut self) -> Result<Box<dyn Expression>, errors::ParseError> {
        let token = self.cur_token.clone();
        self.next();
        let exp = self.parse_expression(Precedence::Prefix);
        if exp.is_err() {
            return Err(errors::ParseError {});
        }
        let right = exp.unwrap();
        Ok(Box::new(PrefixExpression { token, right }))
    }

    fn parse_group_expression(&mut self) -> Result<Box<dyn Expression>, errors::ParseError> {
        self.next(); // consume LPAREN
        let exp = self.parse_expression(Precedence::Lowest);
        if !self.peek_next_is(&Kind::RPAREN) {
            return Err(errors::ParseError {});
        }
        self.next(); // consume RPAREN
        exp
    }

    fn parse_if_expression(&mut self) -> Result<IfExpression, errors::ParseError> {
        let if_token = self.cur_token.clone();
        if !self.expect_next_is(&Kind::LPAREN) {
            return Err(errors::ParseError {});
        } // LPAREN had consumed

        self.next();
        let condition = self.parse_expression(Precedence::Lowest);
        if condition.is_err() {
            return Err(errors::ParseError {});
        }
        let condition = condition.ok().unwrap();

        if !self.expect_next_is(&Kind::RPAREN) {
            return Err(errors::ParseError {});
        } // RPAREN had consumed

        if !self.expect_next_is(&Kind::LBRACE) {
            return Err(errors::ParseError {});
        } // LBRACE had consumed

        let consequence = self.parse_block_statement();

        if consequence.is_err() {
            return Err(errors::ParseError {});
        }
        let consequence = consequence.ok().unwrap();

        let mut alternative = None;
        if self.peek_next_is(&Kind::Else) {
            let res = self.parse_block_statement();
            if res.is_err() {
                return Err(errors::ParseError {});
            }
            alternative = Some(res.ok().unwrap());
        }

        Ok(IfExpression {
            token: if_token,
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_function_literal(&mut self) -> Result<FunctionLiteral, errors::ParseError> {
        let token = self.cur_token.clone();
        let mut ident = None;

        if self.expect_next_is(&Kind::Ident) {
            ident = Some(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            })
        }

        if !self.expect_next_is(&Kind::LPAREN) {
            return Err(errors::ParseError {});
        }

        let params = self.parse_function_parameters();
        if params.is_err() {
            return Err(errors::ParseError {});
        }
        let parameters = params.unwrap();

        if !self.expect_next_is(&Kind::LBRACE) {
            return Err(errors::ParseError {});
        }

        let body = self.parse_block_statement();
        if body.is_err() {
            return Err(errors::ParseError {});
        }
        let body = body.unwrap();

        Ok(FunctionLiteral {
            token,
            ident,
            parameters,
            body,
        })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, errors::ParseError> {
        let mut identifiers = Vec::new();

        if self.peek_next_is(&Kind::RPAREN) {
            self.next(); // consume RPAREN
            return Ok(identifiers);
        }

        if !self.expect_next_is(&Kind::Ident) {
            return Err(errors::ParseError {});
        } // somethings wrong.

        identifiers.push(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        while self.peek_next_is(&Kind::Comma) {
            self.next(); // consume comma
            if !self.expect_next_is(&Kind::Ident) {
                return Err(errors::ParseError {});
            } // got next id

            identifiers.push(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });
        }
        if !self.expect_next_is(&Kind::RPAREN) {
            return Err(errors::ParseError {});
        }

        Ok(identifiers)
    }

    fn parse_call_expression(
        &mut self,
        function: Box<dyn Expression>,
    ) -> Result<CallExpression, errors::ParseError> {
        let token = self.cur_token.clone();
        let arguments = self.parse_call_args();
        if arguments.is_err() {
            return Err(errors::ParseError {});
        }
        let arguments = arguments.unwrap();
        Ok(CallExpression {
            token,
            function,
            arguments,
        })
    }

    fn parse_call_args(&mut self) -> Result<Vec<Box<dyn Expression>>, errors::ParseError> {
        let mut args = Vec::new();
        if self.peek_next_is(&Kind::RPAREN) {
            self.next(); // consume RPAREN
            return Ok(args);
        }
        self.next();

        let res = self.parse_expression(Precedence::Lowest);
        if res.is_err() {
            return Err(errors::ParseError {});
        }

        args.push(res.unwrap());

        while self.peek_next_is(&Kind::Comma) {
            self.next(); // consume Comma
            self.next();

            let res = self.parse_expression(Precedence::Lowest);
            if res.is_err() {
                return Err(errors::ParseError {});
            }

            args.push(res.unwrap());
        }

        if !self.expect_next_is(&Kind::RPAREN) {
            return Err(errors::ParseError {});
        }
        Ok(args)
    }

    fn parse_prefix(
        &mut self,
        kind: &Kind,
    ) -> Result<Box<dyn Expression>, errors::PrefixFunctionError> {
        match kind {
            Kind::Ident => Ok(Box::new(self.parse_identifier())),
            Kind::Int => {
                let res = self.parse_integer_literal();
                if res.is_err() {
                    return Err(errors::PrefixFunctionError::IntegerParseError);
                }
                Ok(Box::new(res.ok().unwrap()))
            }
            Kind::LPAREN => {
                let res = self.parse_group_expression();
                if res.is_err() {
                    return Err(errors::PrefixFunctionError::ParentheseError);
                }
                Ok(res.ok().unwrap())
            }
            Kind::Bang | Kind::Minus => {
                let res = self.parse_prefix_expression();
                if res.is_err() {
                    return Err(errors::PrefixFunctionError::PrefixExpressionError);
                }
                Ok(res.ok().unwrap())
            }
            Kind::True | Kind::False => Ok(Box::new(self.parse_bool_literal())),
            Kind::If => {
                let res = self.parse_if_expression();
                if res.is_err() {
                    return Err(errors::PrefixFunctionError::IfExpressionError);
                }
                Ok(Box::new(res.ok().unwrap()))
            }
            Kind::Function => {
                let res = self.parse_function_literal();
                if res.is_err() {
                    return Err(errors::PrefixFunctionError::FunctionLiteralError);
                }
                Ok(Box::new(res.ok().unwrap()))
            }
            __ => Err(errors::PrefixFunctionError::NoPrefixFunction {}),
        }
    }

    fn parse_infix(
        &mut self,
        left: Box<dyn Expression>,
    ) -> Result<Box<dyn Expression>, errors::InfixFunctionError> {
        let cur_token = self.cur_token.clone();
        if cur_token.kind != Kind::LPAREN {
            let operator = self.cur_token.clone();
            let cur_precedence = self.cur_precedence();

            self.next();

            let right = self.parse_expression(cur_precedence);
            if right.is_err() {
                return Err(errors::InfixFunctionError::ParseError);
            }
            let right = right.ok().unwrap();
            return Ok(Box::new(InfixExpression {
                token: cur_token,
                left,
                operator,
                right,
            }));
        }
        if cur_token.kind == Kind::LPAREN {
            let call_expression = self.parse_call_expression(left);
            if call_expression.is_err() {
                return Err(errors::InfixFunctionError::ParseError);
            }
            return Ok(Box::new(call_expression.unwrap()));
        }
        unreachable!()
    }

    fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Box<dyn Expression>, errors::ParseError> {
        let exp = self.parse_prefix(&self.cur_token.kind.clone());

        if exp.is_err() {
            return Err(errors::ParseError {});
        }

        let mut exp = exp.ok().unwrap();

        while !self.peek_next_is(&Kind::Semicolon) && precedence < self.peek_precedence() {
            if !is_infix(&self.peek_next().kind) {
                return Ok(exp);
            }

            self.next();

            let infix = self.parse_infix(exp);

            if infix.is_err() {
                return Err(errors::ParseError {});
            }
            exp = infix.ok().unwrap();
        }
        Ok(exp)
    }
}
