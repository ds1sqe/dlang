use std::fmt::Debug;

pub enum Object {
    Int(Int),
    Bool(Bool),
    Function(),
    Result,
}

#[derive(Debug)]
pub struct Int {
    pub value: i64,
}

#[derive(Debug)]
pub struct Bool {
    pub value: bool,
}
