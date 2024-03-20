pub mod environment;

use std::fmt::Debug;

use crate::ast::{BlockStatement, Identifier};

use self::environment::Environment;

#[derive(Debug, Clone)]
pub enum Object {
    Return(Return),
    Int(Int),
    Bool(Bool),
    Function(Function),
}

#[derive(PartialEq)]
pub enum ObjectType {
    Return,
    Int,
    Bool,
    Function,
}

pub trait ObjectTrait {
    fn get_type(&self) -> ObjectType;
}

impl ObjectTrait for Object {
    fn get_type(&self) -> ObjectType {
        match self {
            Object::Return(x) => x.get_type(),
            Object::Int(x) => x.get_type(),
            Object::Bool(x) => x.get_type(),
            Object::Function(x) => x.get_type(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Return {
    pub value: Option<Box<Object>>,
}

impl ObjectTrait for Return {
    fn get_type(&self) -> ObjectType {
        ObjectType::Return
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Int {
    pub value: i64,
}
impl ObjectTrait for Int {
    fn get_type(&self) -> ObjectType {
        ObjectType::Int
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bool {
    pub value: bool,
}
impl ObjectTrait for Bool {
    fn get_type(&self) -> ObjectType {
        ObjectType::Bool
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub args: Vec<Identifier>,
    pub block: BlockStatement,
    pub env: Environment<String>,
}
impl ObjectTrait for Function {
    fn get_type(&self) -> ObjectType {
        ObjectType::Function
    }
}

pub fn is_same_type(left: &Object, right: &Object) -> bool {
    left.get_type() == right.get_type()
}
