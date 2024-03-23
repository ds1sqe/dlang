pub mod environment;

use std::fmt::Debug;

use crate::ast::{BlockStatement, Identifier};

use self::environment::Environment;

#[derive(Debug, Clone)]
pub enum Object {
    Return(Return),
    Int(Int),
    Bool(Bool),
    String(StringObject),
    Function(Function),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Return,
    Int,
    Bool,
    String,
    Function,
}

pub trait ObjectTrait {
    fn get_type(&self) -> ObjectType;
    fn to_str(&self) -> String;
}

impl ObjectTrait for Object {
    fn get_type(&self) -> ObjectType {
        match self {
            Object::Return(x) => x.get_type(),
            Object::Int(x) => x.get_type(),
            Object::Bool(x) => x.get_type(),
            Object::Function(x) => x.get_type(),
            Object::String(x) => x.get_type(),
        }
    }

    fn to_str(&self) -> String {
        let inner = match self {
            Object::Return(x) => x.to_str(),
            Object::Int(x) => x.to_str(),
            Object::Bool(x) => x.to_str(),
            Object::String(x) => x.to_str(),
            Object::Function(x) => x.to_str(),
        };
        format!("Object(Enum): {}", inner)
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

    fn to_str(&self) -> String {
        format!("{:?}", self)
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
    fn to_str(&self) -> String {
        format!("Int:{}", self.value)
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
    fn to_str(&self) -> String {
        format!("Bool:{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct StringObject {
    pub value: String,
}
impl ObjectTrait for StringObject {
    fn get_type(&self) -> ObjectType {
        ObjectType::String
    }
    fn to_str(&self) -> String {
        format!("String:{}", self.value)
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
    fn to_str(&self) -> String {
        format!(
            "Function: args <{:?}> block<{:?}> env<{:?}>",
            self.args, self.block, self.env
        )
    }
}

pub fn is_same_type(left: &Object, right: &Object) -> bool {
    left.get_type() == right.get_type()
}
