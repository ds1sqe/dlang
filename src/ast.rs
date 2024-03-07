use std::{any::Any, fmt::Debug};

use crate::token;

pub trait ToAny {
    fn to_any(&self) -> &dyn Any;
}

pub trait Node: Debug + ToAny {
    fn literal(&self) -> String;
    fn to_str(&self) -> String;
}

pub trait Statement: Node + Debug {}

pub trait Expression: Node + Debug {}

impl<T: 'static> ToAny for T {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}
impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
    pub fn push_stm(&mut self, stm: Box<dyn Statement>) {
        self.statements.push(stm);
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token, // token::IDENT
    pub value: String,
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

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: token::Token, // token::Int
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        self.value.clone().to_string()
    }
}
impl Expression for IntegerLiteral {}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub token: token::Token, // token::False or True
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        self.value.clone().to_string()
    }
}
impl Expression for BooleanLiteral {}

#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: token::Token, // token::Function
    pub ident: Option<Identifier>,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        let mut params = Vec::new();
        for param in &self.parameters {
            params.push(param.to_str())
        }
        let mut buf = String::new();
        buf.push_str(&self.literal());
        if self.ident.is_some() {
            buf.push_str(" ");
            buf.push_str(&self.ident.as_ref().unwrap().to_str());
        }
        buf.push_str("(");
        buf.push_str(&params.join(", "));
        buf.push_str(") {");
        buf.push_str(&self.body.to_str());
        buf.push_str("}");
        buf
    }
}
impl Expression for FunctionLiteral {}

#[derive(Debug)]
pub struct LetStatement {
    pub token: token::Token,
    pub identifier: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
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
            buf.push_str(&self.value.as_ref().unwrap().to_str())
        }
        buf.push_str(";");
        buf
    }
}
impl Statement for LetStatement {}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub value: Option<Box<dyn Expression>>,
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

#[derive(Debug)]
pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        let mut buf = String::new();
        for statement in &self.statements {
            buf.push_str(&statement.to_str());
        }
        buf
    }
}
impl Statement for BlockStatement {}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Option<Box<dyn Expression>>,
}
impl Node for ExpressionStatement {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        let mut buf = String::new();
        buf.push_str("");
        if self.expression.is_some() {
            buf.push_str(&self.expression.as_ref().unwrap().to_str());
        }
        buf
    }
}
impl Statement for ExpressionStatement {}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: token::Token,
    pub right: Box<dyn Expression>,
}
impl Node for PrefixExpression {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&self.literal());
        buf.push_str("(");
        buf.push_str(&self.right.to_str());
        buf.push_str(")");
        buf
    }
}
impl Expression for PrefixExpression {}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: token::Token,
    pub left: Box<dyn Expression>,
    pub operator: token::Token,
    pub right: Box<dyn Expression>,
}
impl Node for InfixExpression {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        let mut buf = String::new();
        buf.push_str("(");
        buf.push_str(&self.left.to_str());
        buf.push_str(" ");
        buf.push_str(&self.operator.literal);
        buf.push_str(" ");
        buf.push_str(&self.right.to_str());
        buf.push_str(")");
        buf
    }
}
impl Expression for InfixExpression {}

#[derive(Debug)]
pub struct IfExpression {
    pub token: token::Token, // Token::If
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}
impl Node for IfExpression {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        let mut buf = String::new();
        buf.push_str("if ");
        buf.push_str(&self.condition.to_str());
        buf.push_str(" {");
        buf.push_str(&self.consequence.to_str());
        buf.push_str("}");

        if self.alternative.is_some() {
            buf.push_str(" else {");
            buf.push_str(&self.alternative.as_ref().unwrap().to_str());
            buf.push_str("}");
        }
        buf
    }
}
impl Expression for IfExpression {}

#[derive(Debug)]
pub struct CallExpression {
    pub token: token::Token, // Token::IDENT
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}
impl Node for CallExpression {
    fn literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_str(&self) -> String {
        let mut buf = String::new();

        let mut args = Vec::new();
        for arg in &self.arguments {
            args.push(arg.to_str())
        }

        buf.push_str(&self.function.to_str());
        buf.push_str("(");
        buf.push_str(&args.join(", "));
        buf.push_str(")");
        buf
    }
}
impl Expression for CallExpression {}
