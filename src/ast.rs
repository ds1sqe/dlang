use crate::token;

pub trait Node {
    fn literal(&self) -> String;
    fn to_str(&self) -> String;
}

pub trait Statement: Node {}

pub trait Expression: Node {}

pub struct Program<'a> {
    pub statements: Vec<Box<&'a dyn Statement>>,
}
impl<'a> Program<'a> {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
    pub fn push_stm(mut self, stm: &'a dyn Statement) {
        self.statements.push(Box::new(stm));
    }
}

pub struct Identifier {
    token: token::Token, // token::IDENT
    value: String,
}

impl Node for Identifier {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        self.value.clone()
    }
}
impl Expression for Identifier {}

pub struct LetStatement<'a> {
    token: token::Token,
    identifier: &'a mut Identifier,
    value: Option<Box<dyn Expression>>,
}

impl<'a> Node for LetStatement<'a> {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&self.literal());
        buf.push_str(" ");
        buf.push_str(&self.identifier.to_str());

        if self.value.is_some() {
            buf.push_str(" = ");
            buf.push_str(&self.value.as_ref().unwrap().literal())
        }
        buf.push_str(";");
        buf
    }
}
impl<'a> Statement for LetStatement<'a> {}

pub struct ReturnStatement {
    token: token::Token,
    value: Option<Box<dyn Expression>>,
}

impl Node for ReturnStatement {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&self.literal());
        if self.value.is_some() {
            buf.push_str(" ");
            buf.push_str(&self.value.as_ref().unwrap().literal())
        }
        buf.push_str(";");
        buf
    }
}
impl Statement for ReturnStatement {}

pub struct ExpressionStatement {
    token: token::Token,
    expression: Option<Box<dyn Expression>>,
}
impl Node for ExpressionStatement {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&self.literal());
        if self.expression.is_some() {
            buf.push_str(" = ");
            buf.push_str(&self.expression.as_ref().unwrap().literal())
        }
        buf
    }
}
impl Statement for ExpressionStatement {}
